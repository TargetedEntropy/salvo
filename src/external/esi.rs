use reqwest::Client;
use serde::{Deserialize, Serialize};

const ESI_BASE_URL: &str = "https://esi.evetech.net/latest";

pub struct EsiClient {
    client: Client,
}

impl EsiClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Fetch market prices from ESI
    pub async fn get_market_prices(&self, region_id: i32, type_id: i32) -> anyhow::Result<Vec<MarketOrder>> {
        let url = format!(
            "{}/markets/{}/orders/?type_id={}",
            ESI_BASE_URL, region_id, type_id
        );

        let response = self.client.get(&url).send().await?;
        let orders: Vec<MarketOrder> = response.json().await?;

        Ok(orders)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketOrder {
    pub order_id: i64,
    pub type_id: i32,
    pub price: f64,
    pub volume_remain: i32,
    pub is_buy_order: bool,
}

impl Default for EsiClient {
    fn default() -> Self {
        Self::new()
    }
}
