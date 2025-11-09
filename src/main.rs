mod api;
mod db;
mod error;
mod external;
mod services;

use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "salvo_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database
    let db_pool = db::init_pool().await?;

    // Build application routes
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/api", api::routes())
        .layer(TraceLayer::new_for_http())
        .with_state(db_pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Salvo Backend API - Serpentis Salvage Industrial Planner"
}

async fn health_check() -> &'static str {
    "OK"
}
