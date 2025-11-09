use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    db::{models::*, queries, DbPool},
    error::{ApiError, ApiResult},
    services::{blueprint_matcher, market_data, material_calculator},
};

#[derive(Debug, Deserialize)]
pub struct AnalyzeSalvageRequest {
    pub salvage_items: Vec<SalvageRequestItem>,
    #[serde(default = "default_reprocessing_efficiency")]
    pub reprocessing_efficiency: Option<f64>,
}

fn default_reprocessing_efficiency() -> Option<f64> {
    Some(0.5) // 50% default
}

#[derive(Debug, Deserialize)]
pub struct SalvageRequestItem {
    pub name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeSalvageResponse {
    pub materials: Vec<MaterialOutput>,
    pub total_material_value: f64,
    pub buildable_items: Vec<BuildableItem>,
    pub reprocessing_efficiency_used: f64,
}

#[derive(Debug, Serialize)]
pub struct MaterialOutput {
    pub type_id: i32,
    pub name: String,
    pub quantity: i32,
    pub unit_price: Option<f64>,
    pub total_value: f64,
}

#[derive(Debug, Serialize)]
pub struct BuildableItem {
    pub product_type_id: i32,
    pub product_name: String,
    pub match_percentage: f64,
    pub can_build: bool,
    pub estimated_profit: f64,
    pub profit_margin: f64,
    pub product_price: f64,
    pub material_cost: f64,
    pub missing_materials: Vec<MissingMaterial>,
}

#[derive(Debug, Serialize)]
pub struct MissingMaterial {
    pub type_id: i32,
    pub name: String,
    pub needed: i32,
    pub available: i32,
    pub missing: i32,
    pub cost_to_buy: f64,
}

/// Analyze salvage and determine what can be built
pub async fn analyze_salvage(
    State(pool): State<DbPool>,
    Json(request): Json<AnalyzeSalvageRequest>,
) -> ApiResult<Json<AnalyzeSalvageResponse>> {
    tracing::info!("Analyzing {} salvage items", request.salvage_items.len());

    let reprocessing_efficiency = request.reprocessing_efficiency.unwrap_or(0.5);

    // Step 1: Look up salvage items in database
    let mut salvage_inputs = Vec::new();
    for item in &request.salvage_items {
        let eve_type = queries::get_type_by_name(&pool, &item.name)
            .await
            .map_err(ApiError::Database)?
            .ok_or_else(|| {
                ApiError::NotFound(format!("Salvage item '{}' not found in database", item.name))
            })?;

        salvage_inputs.push(SalvageInput {
            type_id: eve_type.type_id,
            name: eve_type.name,
            quantity: item.quantity,
        });
    }

    // Step 2: Calculate reprocessed materials
    let materials = material_calculator::calculate_materials(
        &pool,
        &salvage_inputs,
        reprocessing_efficiency,
    )
    .await?;

    tracing::info!("Reprocessed into {} different materials", materials.len());

    // Step 3: Get market prices for materials
    let material_type_ids: Vec<i32> = materials.iter().map(|m| m.type_id).collect();
    let mut all_type_ids = material_type_ids.clone();

    // Also need prices for products (will fetch after matching blueprints)

    // Step 4: Match against blueprints
    let blueprint_matches = blueprint_matcher::match_blueprints(&pool, &materials).await?;

    tracing::info!("Found {} blueprint matches", blueprint_matches.len());

    // Collect product type IDs for price lookup
    let product_type_ids: Vec<i32> = blueprint_matches.iter().map(|bm| bm.product.type_id).collect();
    all_type_ids.extend(product_type_ids);

    // Get all prices (materials + products)
    let all_prices = market_data::get_cached_prices(&pool, &all_type_ids).await?;

    // Calculate material outputs with prices
    let mut material_outputs = Vec::new();
    let mut total_material_value = 0.0;

    for material in &materials {
        let unit_price = all_prices
            .get(&material.type_id)
            .and_then(|p| p.sell_price);
        let total_value = unit_price.unwrap_or(0.0) * material.quantity as f64;
        total_material_value += total_value;

        material_outputs.push(MaterialOutput {
            type_id: material.type_id,
            name: material.name.clone(),
            quantity: material.quantity,
            unit_price,
            total_value,
        });
    }

    // Step 5: Calculate profitability for each match
    let mut buildable_items = Vec::new();

    for bp_match in blueprint_matches {
        // Get product price
        let product_price = all_prices
            .get(&bp_match.product.type_id)
            .and_then(|p| p.sell_price)
            .unwrap_or(0.0);

        // Calculate cost of missing materials
        let mut missing_material_cost = 0.0;
        let mut missing_materials_output = Vec::new();

        for missing in &bp_match.missing_materials {
            let unit_price = all_prices
                .get(&missing.type_id)
                .and_then(|p| p.sell_price)
                .unwrap_or(0.0);
            let cost = unit_price * missing.missing as f64;
            missing_material_cost += cost;

            missing_materials_output.push(MissingMaterial {
                type_id: missing.type_id,
                name: missing.name.clone(),
                needed: missing.needed,
                available: missing.available,
                missing: missing.missing,
                cost_to_buy: cost,
            });
        }

        // Industry cost (simplified - could be calculated based on system index)
        let industry_cost = 1000.0; // Placeholder

        let total_cost = missing_material_cost + industry_cost;
        let profit = market_data::calculate_profit(product_price, total_cost, 0.0);
        let profit_margin = market_data::calculate_profit_margin(profit, product_price);

        buildable_items.push(BuildableItem {
            product_type_id: bp_match.product.type_id,
            product_name: bp_match.product.name,
            match_percentage: bp_match.match_percentage,
            can_build: bp_match.match_percentage >= 100.0,
            estimated_profit: profit,
            profit_margin,
            product_price,
            material_cost: total_cost,
            missing_materials: missing_materials_output,
        });
    }

    Ok(Json(AnalyzeSalvageResponse {
        materials: material_outputs,
        total_material_value,
        buildable_items,
        reprocessing_efficiency_used: reprocessing_efficiency,
    }))
}
