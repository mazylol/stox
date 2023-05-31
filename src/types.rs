use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    pub price: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Conversion {
    pub symbol: String,
    pub rate: f64,
    pub amount: f64,
    pub timestamp: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub stocks: Vec<Stock>,
    pub balance: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Stock {
    pub ticker: String,
    pub price: f64,
    pub owned: f64,
}
