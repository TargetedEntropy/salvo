use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const ESI_BASE_URL: &str = "https://esi.evetech.net/latest";

pub struct EsiClient {
    client: Client,
}

impl EsiClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("Salvo-Industrial-Planner/1.0")
                .build()
                .unwrap(),
        }
    }

    /// Fetch market prices from ESI
    /// Returns global average prices
    pub async fn get_market_data(&self, _type_ids: &[i32]) -> anyhow::Result<HashMap<i32, MarketAggregate>> {
        let url = format!("{}/markets/prices/", ESI_BASE_URL);

        let response = self.client.get(&url).send().await?;
        let prices: Vec<EsiPrice> = response.json().await?;

        // Convert to our internal format
        let mut result = HashMap::new();
        for price in prices {
            result.insert(
                price.type_id,
                MarketAggregate {
                    buy: PriceData {
                        weighted_average: price.average_price.unwrap_or(0.0),
                        max: 0.0,
                        min: 0.0,
                        stddev: 0.0,
                        median: 0.0,
                        volume: 0,
                        order_count: 0,
                        percentile: 0.0,
                    },
                    sell: PriceData {
                        weighted_average: price.average_price.unwrap_or(0.0),
                        max: 0.0,
                        min: 0.0,
                        stddev: 0.0,
                        median: 0.0,
                        volume: 0,
                        order_count: 0,
                        percentile: 0.0,
                    },
                },
            );
        }

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct EsiPrice {
    #[serde(rename = "type_id")]
    pub type_id: i32,
    #[serde(rename = "average_price")]
    pub average_price: Option<f64>,
    #[serde(rename = "adjusted_price")]
    pub adjusted_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketAggregate {
    pub buy: PriceData,
    pub sell: PriceData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceData {
    #[serde(rename = "weightedAverage")]
    pub weighted_average: f64,
    pub max: f64,
    pub min: f64,
    pub stddev: f64,
    pub median: f64,
    pub volume: i64,
    #[serde(rename = "orderCount")]
    pub order_count: i64,
    pub percentile: f64,
}

// Re-export with old field names for backwards compatibility
impl PriceData {
    #[allow(non_snake_case)]
    pub fn weightedAverage(&self) -> f64 {
        self.weighted_average
    }

    #[allow(non_snake_case)]
    pub fn orderCount(&self) -> i64 {
        self.order_count
    }
}

impl Default for EsiClient {
    fn default() -> Self {
        Self::new()
    }
}

// Backwards compatibility alias
pub type FuzzworksClient = EsiClient;
