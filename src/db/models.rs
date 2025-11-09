use serde::{Deserialize, Serialize};

/// EVE Online type definition (items, materials, salvage, blueprints)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EveType {
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub group_id: Option<i32>,
    pub category_id: Option<i32>,
    pub volume: Option<f64>,
    pub base_price: Option<f64>,
    pub market_group_id: Option<i32>,
    pub portion_size: Option<i32>,
}

/// Material reprocessing yield
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MaterialReprocessing {
    pub id: i32,
    pub source_type_id: i32,
    pub material_type_id: i32,
    pub quantity: i32,
}

/// Blueprint definition
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Blueprint {
    pub blueprint_type_id: i32,
    pub product_type_id: i32,
    pub manufacturing_time: Option<i32>,
    pub max_production_limit: Option<i32>,
}

/// Material requirement for a blueprint
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BlueprintMaterial {
    pub id: i32,
    pub blueprint_type_id: i32,
    pub material_type_id: i32,
    pub quantity: i32,
}

/// Market price data
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MarketPrice {
    pub type_id: i32,
    pub region_id: i32,
    pub sell_price: Option<f64>,
    pub buy_price: Option<f64>,
    pub daily_volume: Option<i32>,
    pub updated_at: String, // SQLite DATETIME as string
}

// ===== Working models for business logic =====

/// Represents a material with quantity (for calculations)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub type_id: i32,
    pub name: String,
    pub quantity: i32,
}

/// Input salvage item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalvageInput {
    pub type_id: i32,
    pub name: String,
    pub quantity: i32,
}
