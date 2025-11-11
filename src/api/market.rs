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

/// Update market prices from ESI API
/// If type_ids is empty, fetches and stores ALL prices from ESI (bulk backfill)
pub async fn update_market_prices(
    State(pool): State<DbPool>,
    Json(request): Json<UpdateMarketPricesRequest>,
) -> ApiResult<Json<UpdateMarketPricesResponse>> {
    let esi_client = FuzzworksClient::new();
    let market_data = esi_client
        .get_market_data(&request.type_ids)
        .await
        .map_err(|e| ApiError::ExternalApi(format!("ESI API error: {}", e)))?;

    let mut updated_count = 0;
    let mut prices_output = Vec::new();

    // If type_ids is empty, store ALL prices from ESI (backfill mode)
    // Otherwise, only store requested type_ids
    let type_ids_to_process: Vec<i32> = if request.type_ids.is_empty() {
        tracing::info!("Backfilling ALL market prices from ESI");
        market_data.keys().copied().collect()
    } else {
        tracing::info!("Fetching market prices for {} types", request.type_ids.len());
        request.type_ids.clone()
    };

    for type_id in &type_ids_to_process {
        if let Some(aggregate) = market_data.get(type_id) {
            let market_price = MarketPrice {
                type_id: *type_id,
                region_id: 10000002, // Global average (not region-specific)
                sell_price: Some(aggregate.sell.weighted_average),
                buy_price: Some(aggregate.buy.weighted_average),
                daily_volume: Some(aggregate.sell.volume as i32),
                updated_at: chrono::Utc::now().to_rfc3339(),
            };

            // Try to insert, but skip if the type_id doesn't exist in eve_types (foreign key constraint)
            match queries::upsert_market_price(&pool, &market_price).await {
                Ok(_) => {
                    // Only return individual prices if not in backfill mode (to avoid huge response)
                    if !request.type_ids.is_empty() {
                        prices_output.push(MarketPriceOutput {
                            type_id: *type_id,
                            sell_price: market_price.sell_price,
                            buy_price: market_price.buy_price,
                        });
                    }

                    updated_count += 1;

                    // Log progress every 1000 items in backfill mode
                    if request.type_ids.is_empty() && updated_count % 1000 == 0 {
                        tracing::info!("Backfilled {} prices...", updated_count);
                    }
                }
                Err(e) => {
                    // Skip foreign key constraint errors (type doesn't exist in our database)
                    let err_str = format!("{:?}", e);
                    if !err_str.contains("FOREIGN KEY constraint") {
                        // If it's not a foreign key error, return it
                        return Err(ApiError::Database(e));
                    }
                    // Silently skip foreign key errors
                }
            }
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
