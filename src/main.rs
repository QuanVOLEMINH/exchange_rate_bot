mod services;

use dotenv::dotenv;
use services::exchange_rate::{
    exchange_rate_response::Currency, exchange_rate_service::ExchangeRateService,
};
use std::env;

#[tokio::main]
async fn main() {
    println!("Hello!");
    dotenv().ok();

    get_exchange_rate().await;
}

async fn get_exchange_rate() {
    let api_base_url =
        env::var("EXCHANGERATE_API_BASE_URL").expect("EXCHANGERATE_API_BASE_URL must be set");
    let api_key = env::var("EXCHANGERATE_API_KEY").expect("EXCHANGERATE_API_KEY must be set");
    let exchange_rate_service = ExchangeRateService::new(api_base_url, api_key);

    let symbol = Currency::GBP;
    match exchange_rate_service.get_exchange_rate(&symbol).await {
        Ok(data) => println!("Exchange rate data for {}: {:?}", symbol, data),
        Err(e) => println!("Error fetching exchange rate: {}", e),
    }
}
