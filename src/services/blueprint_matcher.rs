use crate::db::models::{Blueprint, BlueprintMaterial, Material};

pub struct BlueprintMatch {
    pub blueprint: Blueprint,
    pub match_percentage: f64,
    pub missing_materials: Vec<Material>,
}

/// Match available materials against blueprint requirements
pub fn match_blueprints(
    available_materials: &[Material],
    blueprints: &[Blueprint],
    blueprint_materials: &[BlueprintMaterial],
) -> Vec<BlueprintMatch> {
    // TODO: Implement blueprint matching logic
    // 1. For each blueprint, check material requirements
    // 2. Calculate match percentage (available / required)
    // 3. Identify missing materials
    // 4. Return sorted by match percentage
    vec![]
}

/// Calculate if a blueprint can be fully built with available materials
pub fn can_build(
    blueprint: &Blueprint,
    available_materials: &[Material],
    blueprint_materials: &[BlueprintMaterial],
) -> bool {
    // Get materials needed for this blueprint
    let required: Vec<&BlueprintMaterial> = blueprint_materials
        .iter()
        .filter(|bm| bm.blueprint_type_id == blueprint.blueprint_type_id)
        .collect();

    // Check if we have enough of each material
    required.iter().all(|req| {
        available_materials
            .iter()
            .find(|m| m.type_id == req.material_type_id)
            .map(|m| m.quantity >= req.quantity)
            .unwrap_or(false)
    })
}
