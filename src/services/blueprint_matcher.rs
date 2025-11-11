use crate::db::{models::*, queries, DbPool};
use crate::error::{ApiError, ApiResult};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct BlueprintMatch {
    pub blueprint: Blueprint,
    pub product: EveType,
    pub match_percentage: f64,
    pub missing_materials: Vec<MaterialWithName>,
    pub satisfied_count: usize,
    pub total_count: usize,
}

#[derive(Debug, Clone)]
pub struct MaterialWithName {
    pub type_id: i32,
    pub name: String,
    pub needed: i32,
    pub available: i32,
    pub missing: i32,
}

/// Match available materials against all blueprints
pub async fn match_blueprints(
    pool: &DbPool,
    available_materials: &[Material],
) -> ApiResult<Vec<BlueprintMatch>> {
    // Get all blueprints
    let blueprints = queries::get_all_blueprints(pool)
        .await
        .map_err(ApiError::Database)?;

    // Get all blueprint materials
    let all_bp_materials = queries::get_all_blueprint_materials(pool)
        .await
        .map_err(ApiError::Database)?;

    // Create lookup map for available materials
    let available_map: HashMap<i32, i32> = available_materials
        .iter()
        .map(|m| (m.type_id, m.quantity))
        .collect();

    let mut matches = Vec::new();

    for blueprint in blueprints {
        // Get materials required for this blueprint
        let required: Vec<&BlueprintMaterial> = all_bp_materials
            .iter()
            .filter(|bm| bm.blueprint_type_id == blueprint.blueprint_type_id)
            .collect();

        if required.is_empty() {
            continue;
        }

        let mut satisfied_count = 0;
        let total_count = required.len();
        let mut missing_materials = Vec::new();

        for req in &required {
            let available = available_map.get(&req.material_type_id).copied().unwrap_or(0);
            let needed = req.quantity;

            if available >= needed {
                satisfied_count += 1;
            } else {
                let missing = needed - available;

                // Get material name
                let material_type = queries::get_type_by_id(pool, req.material_type_id)
                    .await
                    .map_err(ApiError::Database)?
                    .ok_or_else(|| {
                        ApiError::NotFound(format!("Material type {} not found", req.material_type_id))
                    })?;

                missing_materials.push(MaterialWithName {
                    type_id: req.material_type_id,
                    name: material_type.name,
                    needed,
                    available,
                    missing,
                });
            }
        }

        let match_percentage = (satisfied_count as f64 / total_count as f64) * 100.0;

        // Get product info - skip blueprints with invalid/deprecated product types
        let product = match queries::get_type_by_id(pool, blueprint.product_type_id).await {
            Ok(Some(p)) => p,
            Ok(None) => {
                // Product type not found - this is a deprecated/removed item, skip this blueprint
                tracing::warn!(
                    "Skipping blueprint {} - product type {} not found (deprecated item)",
                    blueprint.blueprint_type_id,
                    blueprint.product_type_id
                );
                continue;
            }
            Err(e) => return Err(ApiError::Database(e)),
        };

        matches.push(BlueprintMatch {
            blueprint,
            product,
            match_percentage,
            missing_materials,
            satisfied_count,
            total_count,
        });
    }

    // Sort by match percentage (highest first), then by product name
    matches.sort_by(|a, b| {
        b.match_percentage
            .partial_cmp(&a.match_percentage)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.product.name.cmp(&b.product.name))
    });

    Ok(matches)
}

/// Calculate if a blueprint can be fully built with available materials
pub fn can_build(
    available_materials: &HashMap<i32, i32>,
    blueprint_materials: &[BlueprintMaterial],
) -> bool {
    blueprint_materials.iter().all(|req| {
        available_materials
            .get(&req.material_type_id)
            .map(|&available| available >= req.quantity)
            .unwrap_or(false)
    })
}
