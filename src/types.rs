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
