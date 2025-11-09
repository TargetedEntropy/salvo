pub mod salvage;

use axum::{routing::post, Router};
use crate::db::DbPool;

pub fn routes() -> Router<DbPool> {
    Router::new()
        .route("/salvage/analyze", post(salvage::analyze_salvage))
}
