use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    db::{models::SalvageItem, DbPool},
    error::ApiResult,
};

#[derive(Debug, Deserialize)]
pub struct AnalyzeSalvageRequest {
    pub salvage_items: Vec<SalvageInput>,
}

#[derive(Debug, Deserialize)]
pub struct SalvageInput {
    pub name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeSalvageResponse {
    pub materials: Vec<MaterialOutput>,
    pub buildable_items: Vec<BuildableItem>,
}

#[derive(Debug, Serialize)]
pub struct MaterialOutput {
    pub name: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize)]
pub struct BuildableItem {
    pub name: String,
    pub match_percentage: f64,
    pub estimated_profit: f64,
    pub missing_materials: Vec<MaterialOutput>,
}

/// Analyze salvage and determine what can be built
pub async fn analyze_salvage(
    State(_pool): State<DbPool>,
    Json(request): Json<AnalyzeSalvageRequest>,
) -> ApiResult<Json<AnalyzeSalvageResponse>> {
    tracing::info!("Analyzing {} salvage items", request.salvage_items.len());

    // TODO: Implement salvage analysis logic
    // 1. Look up salvage items in SDE
    // 2. Calculate reprocessed materials
    // 3. Match against blueprints
    // 4. Fetch market prices
    // 5. Calculate profitability

    Ok(Json(AnalyzeSalvageResponse {
        materials: vec![],
        buildable_items: vec![],
    }))
}
