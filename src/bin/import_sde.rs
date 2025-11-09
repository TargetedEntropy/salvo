/// SDE Import Tool
///
/// This tool imports data from EVE Online's Static Data Export (SDE) into the Salvo database.
///
/// Usage:
///   cargo run --bin import_sde -- --sde-path /path/to/sde
///
/// The SDE can be downloaded from: https://developers.eveonline.com/resources/downloads

use anyhow::{Context, Result};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct TypeId {
    #[serde(rename = "groupID")]
    group_id: Option<i32>,
    #[serde(rename = "name")]
    name: HashMap<String, String>,
    description: Option<HashMap<String, String>>,
    volume: Option<f64>,
    #[serde(rename = "basePrice")]
    base_price: Option<f64>,
    #[serde(rename = "marketGroupID")]
    market_group_id: Option<i32>,
    #[serde(rename = "portionSize")]
    portion_size: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct BlueprintActivity {
    materials: Option<Vec<Material>>,
    products: Option<Vec<Product>>,
    time: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct Material {
    #[serde(rename = "typeID")]
    type_id: i32,
    quantity: i32,
}

#[derive(Debug, Deserialize)]
struct Product {
    #[serde(rename = "typeID")]
    type_id: i32,
    quantity: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Salvo SDE Import Tool ===\n");

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let sde_path = if args.len() > 2 && args[1] == "--sde-path" {
        PathBuf::from(&args[2])
    } else {
        println!("Usage: cargo run --bin import_sde -- --sde-path /path/to/sde");
        println!("\nThe SDE can be downloaded from:");
        println!("https://developers.eveonline.com/resources/downloads\n");
        return Ok(());
    };

    if !sde_path.exists() {
        anyhow::bail!("SDE path does not exist: {}", sde_path.display());
    }

    // Load environment and connect to database
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:salvo.db".to_string());

    println!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    println!("Database migrations complete\n");

    // Import type IDs (items)
    import_type_ids(&pool, &sde_path).await?;

    // Import blueprints
    import_blueprints(&pool, &sde_path).await?;

    println!("\n=== Import Complete ===");
    Ok(())
}

async fn import_type_ids(pool: &SqlitePool, sde_path: &PathBuf) -> Result<()> {
    println!("Importing type IDs...");

    let types_path = sde_path.join("fsd").join("typeIDs.yaml");
    if !types_path.exists() {
        println!("⚠️  typeIDs.yaml not found, skipping");
        return Ok(());
    }

    let yaml_content = fs::read_to_string(&types_path)
        .context("Failed to read typeIDs.yaml")?;

    // Parse YAML
    let types: HashMap<i32, TypeId> = serde_yaml::from_str(&yaml_content)
        .context("Failed to parse typeIDs.yaml")?;

    println!("Found {} type definitions", types.len());

    let mut imported = 0;
    for (type_id, type_data) in types {
        // Get English name
        let name = type_data.name.get("en")
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("Unknown {}", type_id));

        let description = type_data.description
            .as_ref()
            .and_then(|d| d.get("en"))
            .map(|s| s.as_str());

        // Insert into database
        sqlx::query(
            "INSERT OR IGNORE INTO eve_types
             (type_id, name, description, group_id, volume, base_price, market_group_id, portion_size)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(type_id)
        .bind(&name)
        .bind(description)
        .bind(type_data.group_id)
        .bind(type_data.volume)
        .bind(type_data.base_price)
        .bind(type_data.market_group_id)
        .bind(type_data.portion_size)
        .execute(pool)
        .await?;

        imported += 1;

        if imported % 1000 == 0 {
            print!(".");
            std::io::Write::flush(&mut std::io::stdout())?;
        }
    }

    println!("\n✓ Imported {} type IDs", imported);
    Ok(())
}

async fn import_blueprints(_pool: &SqlitePool, sde_path: &PathBuf) -> Result<()> {
    println!("\nImporting blueprints...");

    let blueprints_path = sde_path.join("fsd").join("blueprints.yaml");
    if !blueprints_path.exists() {
        println!("⚠️  blueprints.yaml not found, skipping");
        return Ok(());
    }

    let _yaml_content = fs::read_to_string(&blueprints_path)
        .context("Failed to read blueprints.yaml")?;

    // Parse YAML (simplified - full SDE structure is more complex)
    println!("Parsing blueprints.yaml (this may take a while)...");

    // Note: The actual SDE blueprint format is complex
    // This is a simplified version - real implementation would need proper SDE parsing
    println!("⚠️  Blueprint import requires full SDE parser (complex YAML structure)");
    println!("   Using sample data for now. Full implementation TODO.");

    Ok(())
}
