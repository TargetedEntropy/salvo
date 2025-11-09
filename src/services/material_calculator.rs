use crate::db::{models::*, queries, DbPool};
use crate::error::{ApiError, ApiResult};
use std::collections::HashMap;

/// Calculate materials from reprocessed salvage
pub async fn calculate_materials(
    pool: &DbPool,
    salvage_items: &[SalvageInput],
    reprocessing_efficiency: f64,
) -> ApiResult<Vec<Material>> {
    let mut material_totals: HashMap<i32, i32> = HashMap::new();

    for salvage in salvage_items {
        // Get reprocessing yields for this salvage item
        let yields = queries::get_reprocessing_yields(pool, salvage.type_id)
            .await
            .map_err(ApiError::Database)?;

        // Apply reprocessing efficiency and salvage quantity
        for yield_entry in yields {
            let material_quantity = (yield_entry.quantity as f64
                * reprocessing_efficiency
                * salvage.quantity as f64) as i32;

            *material_totals.entry(yield_entry.material_type_id).or_insert(0) += material_quantity;
        }
    }

    // Convert to Material structs with names
    let mut materials = Vec::new();
    for (type_id, quantity) in material_totals {
        let eve_type = queries::get_type_by_id(pool, type_id)
            .await
            .map_err(ApiError::Database)?
            .ok_or_else(|| ApiError::NotFound(format!("Material type {} not found", type_id)))?;

        materials.push(Material {
            type_id,
            name: eve_type.name,
            quantity,
        });
    }

    // Sort by type_id for consistent output
    materials.sort_by_key(|m| m.type_id);

    Ok(materials)
}

/// Calculate reprocessing efficiency based on skills
/// Default efficiency with no skills: 50%
/// Perfect skills (all level 5): ~84.4%
pub fn calculate_reprocessing_efficiency(
    reprocessing_skill: u8,
    reprocessing_efficiency_skill: u8,
    specific_processing_skill: u8, // e.g., Scrapmetal Processing
) -> f64 {
    // Base formula from EVE Online
    // Base = 0.5 * (1 + 0.03 * Reprocessing) * (1 + 0.02 * ReprocessingEfficiency) * (1 + 0.02 * SpecificProcessing)
    let base = 0.5;
    let reprocessing_bonus = 1.0 + (0.03 * reprocessing_skill as f64);
    let efficiency_bonus = 1.0 + (0.02 * reprocessing_efficiency_skill as f64);
    let specific_bonus = 1.0 + (0.02 * specific_processing_skill as f64);

    base * reprocessing_bonus * efficiency_bonus * specific_bonus
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reprocessing_efficiency_no_skills() {
        let efficiency = calculate_reprocessing_efficiency(0, 0, 0);
        assert_eq!(efficiency, 0.5);
    }

    #[test]
    fn test_reprocessing_efficiency_max_skills() {
        let efficiency = calculate_reprocessing_efficiency(5, 5, 5);
        // 0.5 * 1.15 * 1.10 * 1.10 = 0.69575
        assert!((efficiency - 0.69575).abs() < 0.0001);
    }
}
