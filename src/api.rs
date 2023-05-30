use crate::fs::Config;

use crate::types::{Conversion, Price};

pub async fn get_price(config: &Config, ticker: String) -> f64 {
    let response: Price = reqwest::Client::new()
        .get("https://twelve-data1.p.rapidapi.com/price")
        .header("X-RapidAPI-Key", &config.api_key)
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

pub async fn get_conversion(config: &Config, from: String, to: String, amount: f64) -> f64 {
    let response: Conversion = reqwest::Client::new()
        .get("https://twelve-data1.p.rapidapi.com/currency_conversion")
        .header("X-RapidAPI-Key", &config.api_key)
        .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
        .query(&[("symbol", from + "/" + &to), ("amount", amount.to_string())])
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap_or(Conversion {
            symbol: String::from(""),
            rate: 0.0,
            amount: 0.0,
            timestamp: 0,
        });

    return response.amount;
}
