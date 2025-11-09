use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::{
    db::{models::MarketPrice, queries, DbPool},
    error::{ApiError, ApiResult},
    external::fuzzworks::FuzzworksClient,
};

#[derive(Debug, Deserialize)]
pub struct UpdateMarketPricesRequest {
    pub type_ids: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct UpdateMarketPricesResponse {
    pub updated_count: usize,
    pub prices: Vec<MarketPriceOutput>,
}

#[derive(Debug, Serialize)]
pub struct MarketPriceOutput {
    pub type_id: i32,
    pub sell_price: Option<f64>,
    pub buy_price: Option<f64>,
}

/// Update market prices from Fuzzworks API
pub async fn update_market_prices(
    State(pool): State<DbPool>,
    Json(request): Json<UpdateMarketPricesRequest>,
) -> ApiResult<Json<UpdateMarketPricesResponse>> {
    if request.type_ids.is_empty() {
        return Err(ApiError::InvalidInput(
            "type_ids cannot be empty".to_string(),
        ));
    }

    tracing::info!("Fetching market prices for {} types", request.type_ids.len());

    let fuzzworks = FuzzworksClient::new();
    let market_data = fuzzworks
        .get_market_data(&request.type_ids)
        .await
        .map_err(|e| ApiError::ExternalApi(format!("Fuzzworks API error: {}", e)))?;

    let mut updated_count = 0;
    let mut prices_output = Vec::new();

    for type_id in &request.type_ids {
        if let Some(aggregate) = market_data.get(type_id) {
            let market_price = MarketPrice {
                type_id: *type_id,
                region_id: 10000002, // The Forge (Jita)
                sell_price: Some(aggregate.sell.weighted_average),
                buy_price: Some(aggregate.buy.weighted_average),
                daily_volume: Some(aggregate.sell.volume as i32),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };

            queries::upsert_market_price(&pool, &market_price)
                .await
                .map_err(ApiError::Database)?;

            prices_output.push(MarketPriceOutput {
                type_id: *type_id,
                sell_price: market_price.sell_price,
                buy_price: market_price.buy_price,
            });

            updated_count += 1;
        }
    }

    tracing::info!("Updated {} market prices", updated_count);

    Ok(Json(UpdateMarketPricesResponse {
        updated_count,
        prices: prices_output,
    }))
}

/// Get cached market prices
pub async fn get_market_prices(
    State(pool): State<DbPool>,
    Json(request): Json<UpdateMarketPricesRequest>,
) -> ApiResult<Json<Vec<MarketPrice>>> {
    let prices = queries::get_market_prices(&pool, &request.type_ids)
        .await
        .map_err(ApiError::Database)?;

    Ok(Json(prices))
}
