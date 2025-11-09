use crate::db::{models::*, queries, DbPool};
use crate::error::{ApiError, ApiResult};
use std::collections::HashMap;

/// Get market prices for type IDs (from database cache)
pub async fn get_cached_prices(
    pool: &DbPool,
    type_ids: &[i32],
) -> ApiResult<HashMap<i32, MarketPrice>> {
    let prices = queries::get_market_prices(pool, type_ids)
        .await
        .map_err(ApiError::Database)?;

    let price_map: HashMap<i32, MarketPrice> =
        prices.into_iter().map(|p| (p.type_id, p)).collect();

    Ok(price_map)
}

/// Calculate material costs from cached market prices
pub async fn calculate_material_costs(
    pool: &DbPool,
    materials: &[Material],
) -> ApiResult<f64> {
    let type_ids: Vec<i32> = materials.iter().map(|m| m.type_id).collect();
    let prices = get_cached_prices(pool, &type_ids).await?;

    let total_cost: f64 = materials
        .iter()
        .map(|material| {
            prices
                .get(&material.type_id)
                .and_then(|p| p.sell_price)
                .unwrap_or(0.0)
                * material.quantity as f64
        })
        .sum();

    Ok(total_cost)
}

/// Calculate profit for building an item
pub fn calculate_profit(
    product_sell_price: f64,
    material_costs: f64,
    industry_cost: f64,
) -> f64 {
    product_sell_price - material_costs - industry_cost
}

/// Calculate profit margin percentage
pub fn calculate_profit_margin(profit: f64, product_sell_price: f64) -> f64 {
    if product_sell_price == 0.0 {
        0.0
    } else {
        (profit / product_sell_price) * 100.0
    }
}
