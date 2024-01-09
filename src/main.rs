mod services;

use services::exchange_rate::exchange_rate_service::ExchangeRateService;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    println!("Hello!");
    dotenv().ok();

    get_exchange_rate().await;
}

async fn get_exchange_rate() {
    let api_key = env::var("EXCHANGERATE_API_KEY").expect("EXCHANGERATE_API_KEY must be set");
    let exchange_rate_service = ExchangeRateService::new(api_key);

    let symbol = "GBP";
    match exchange_rate_service.get_exchange_rate(symbol).await {
        Ok(data) => println!("Exchange rate data for {}: {:?}", symbol, data),
        Err(e) => println!("Error fetching exchange rate: {}", e),
    }
}
