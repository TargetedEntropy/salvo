pub mod market;
pub mod salvage;

use axum::{routing::{get, post}, Router};
use crate::db::DbPool;

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/salvage/analyze", post(salvage::analyze_salvage))
        .route("/market/update", post(market::update_market_prices))
        .route("/market/prices", post(market::get_market_prices))
}
