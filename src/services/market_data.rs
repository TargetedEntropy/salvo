use crate::db::models::MarketPrice;
use crate::error::ApiError;

/// Fetch market prices from external API
pub async fn fetch_market_prices(type_ids: &[i32]) -> Result<Vec<MarketPrice>, ApiError> {
    // TODO: Implement market data fetching
    // - Call Fuzzworks or ESI API
    // - Parse response
    // - Return market prices
    Ok(vec![])
}

/// Calculate profit for building an item
pub fn calculate_profit(
    product_price: f64,
    material_costs: f64,
    industry_cost: f64,
) -> f64 {
    product_price - material_costs - industry_cost
}
