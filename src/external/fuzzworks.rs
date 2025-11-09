use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const FUZZWORKS_BASE_URL: &str = "https://market.fuzzwork.co.uk/aggregates";

pub struct FuzzworksClient {
    client: Client,
}

impl FuzzworksClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetch aggregated market data from Fuzzworks
    /// Region 10000002 = The Forge (Jita)
    pub async fn get_market_data(&self, type_ids: &[i32]) -> anyhow::Result<HashMap<i32, MarketAggregate>> {
        let type_ids_str: Vec<String> = type_ids.iter().map(|id| id.to_string()).collect();
        let url = format!(
            "{}/?region=10000002&types={}",
            FUZZWORKS_BASE_URL,
            type_ids_str.join(",")
        );

        let response = self.client.get(&url).send().await?;
        let data: HashMap<String, MarketAggregate> = response.json().await?;

        // Convert string keys to i32
        let result: HashMap<i32, MarketAggregate> = data
            .into_iter()
            .filter_map(|(k, v)| k.parse::<i32>().ok().map(|type_id| (type_id, v)))
            .collect();

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketAggregate {
    pub buy: PriceData,
    pub sell: PriceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    pub weightedAverage: f64,
    pub max: f64,
    pub min: f64,
    pub stddev: f64,
    pub median: f64,
    pub volume: i64,
    pub orderCount: i64,
    pub percentile: f64,
}

impl Default for FuzzworksClient {
    fn default() -> Self {
        Self::new()
    }
}
