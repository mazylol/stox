use serde::{Deserialize, Serialize};

use crate::fs::Config;

pub async fn get_price(config: Config, ticker: String) -> f64 {
    let response: Price = reqwest::Client::new()
        .get("https://twelve-data1.p.rapidapi.com/price")
        .header("X-RapidAPI-Key", config.api_key)
        .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
        .query(&[("symbol", ticker)])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap_or(Price {
            price: String::from("0"),
        });

    return response.price.parse::<f64>().unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    price: String,
}
