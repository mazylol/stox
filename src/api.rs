use crate::fs::Config;

pub async fn get_price(config: Config, ticker: String) -> f64 {
    let response = reqwest::Client::new()
        .get("https://twelve-data1.p.rapidapi.com/price")
        .header("X-RapidAPI-Key", config.api_key)
        .header("X-RapidAPI-Host", "twelve-data1.p.rapidapi.com")
        .query(&[("symbol", ticker)])
        .send()
        .await;

    let json = response.unwrap().json::<serde_json::Value>().await.unwrap();
    let price = json["price"].as_f64().unwrap();
    return price;
}