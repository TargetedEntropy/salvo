use super::models::*;
use super::DbPool;
use sqlx;

/// Look up EVE type by name
pub async fn get_type_by_name(pool: &DbPool, name: &str) -> sqlx::Result<Option<EveType>> {
    sqlx::query_as::<_, EveType>(
        "SELECT * FROM eve_types WHERE name = ? LIMIT 1"
    )
    .bind(name)
    .fetch_optional(pool)
    .await
}

/// Look up EVE type by type_id
pub async fn get_type_by_id(pool: &DbPool, type_id: i32) -> sqlx::Result<Option<EveType>> {
    sqlx::query_as::<_, EveType>(
        "SELECT * FROM eve_types WHERE type_id = ? LIMIT 1"
    )
    .bind(type_id)
    .fetch_optional(pool)
    .await
}

/// Get reprocessing yields for a salvage item
pub async fn get_reprocessing_yields(
    pool: &DbPool,
    source_type_id: i32,
) -> sqlx::Result<Vec<MaterialReprocessing>> {
    sqlx::query_as::<_, MaterialReprocessing>(
        "SELECT * FROM material_reprocessing WHERE source_type_id = ?"
    )
    .bind(source_type_id)
    .fetch_all(pool)
    .await
}

/// Get all blueprints
pub async fn get_all_blueprints(pool: &DbPool) -> sqlx::Result<Vec<Blueprint>> {
    sqlx::query_as::<_, Blueprint>("SELECT * FROM blueprints")
        .fetch_all(pool)
        .await
}

/// Get blueprint by product type
pub async fn get_blueprints_for_product(
    pool: &DbPool,
    product_type_id: i32,
) -> sqlx::Result<Vec<Blueprint>> {
    sqlx::query_as::<_, Blueprint>(
        "SELECT * FROM blueprints WHERE product_type_id = ?"
    )
    .bind(product_type_id)
    .fetch_all(pool)
    .await
}

/// Get material requirements for a blueprint
pub async fn get_blueprint_materials(
    pool: &DbPool,
    blueprint_type_id: i32,
) -> sqlx::Result<Vec<BlueprintMaterial>> {
    sqlx::query_as::<_, BlueprintMaterial>(
        "SELECT * FROM blueprint_materials WHERE blueprint_type_id = ?"
    )
    .bind(blueprint_type_id)
    .fetch_all(pool)
    .await
}

/// Get all blueprint materials for multiple blueprints
pub async fn get_all_blueprint_materials(pool: &DbPool) -> sqlx::Result<Vec<BlueprintMaterial>> {
    sqlx::query_as::<_, BlueprintMaterial>("SELECT * FROM blueprint_materials")
        .fetch_all(pool)
        .await
}

/// Get market price for a type
pub async fn get_market_price(pool: &DbPool, type_id: i32) -> sqlx::Result<Option<MarketPrice>> {
    sqlx::query_as::<_, MarketPrice>(
        "SELECT * FROM market_prices WHERE type_id = ? LIMIT 1"
    )
    .bind(type_id)
    .fetch_optional(pool)
    .await
}

/// Get market prices for multiple types
pub async fn get_market_prices(pool: &DbPool, type_ids: &[i32]) -> sqlx::Result<Vec<MarketPrice>> {
    if type_ids.is_empty() {
        return Ok(vec![]);
    }

    let placeholders = type_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("SELECT * FROM market_prices WHERE type_id IN ({})", placeholders);

    let mut query = sqlx::query_as::<_, MarketPrice>(&query);
    for type_id in type_ids {
        query = query.bind(type_id);
    }

    query.fetch_all(pool).await
}

/// Upsert market price
pub async fn upsert_market_price(pool: &DbPool, price: &MarketPrice) -> sqlx::Result<()> {
    sqlx::query(
        "INSERT INTO market_prices (type_id, region_id, sell_price, buy_price, daily_volume, updated_at)
         VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP)
         ON CONFLICT(type_id) DO UPDATE SET
            sell_price = excluded.sell_price,
            buy_price = excluded.buy_price,
            daily_volume = excluded.daily_volume,
            updated_at = CURRENT_TIMESTAMP"
    )
    .bind(price.type_id)
    .bind(price.region_id)
    .bind(price.sell_price)
    .bind(price.buy_price)
    .bind(price.daily_volume)
    .execute(pool)
    .await?;

    Ok(())
}
