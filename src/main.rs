mod services;

use dotenv::dotenv;
use services::exchange_rate::{
    exchange_rate_response::Currency, exchange_rate_service::ExchangeRateService,
};
use std::{env, collections::HashMap, process};

#[tokio::main]
async fn main() {
    println!("Starting the program...");
    dotenv().ok();

    let message = construct_exchange_rate_message().await;
    println!("{}", message);
    println!("Completed!");
}

async fn construct_exchange_rate_message() -> String {
    let api_base_url =
        env::var("EXCHANGERATE_API_BASE_URL").expect("EXCHANGERATE_API_BASE_URL must be set");
    let api_key = env::var("EXCHANGERATE_API_KEY").expect("EXCHANGERATE_API_KEY must be set");
    let exchange_rate_service = ExchangeRateService::new(api_base_url, api_key);

    let base_currencies = vec![Currency::GBP, Currency::EUR, Currency::USD];
    let mut rates = HashMap::new();

    for base in &base_currencies {
        match exchange_rate_service.get_exchange_rate(base).await {
            Ok(data) => { rates.insert(base, data.conversion_rates); },
            Err(e) => {
                eprintln!("Error fetching exchange rate for {:?}: {}", base, e);
                process::exit(1); // Exits the program
            },
        }
    }

    // Construct the message
    let message = format!(
        "GBP to VND: {:.3}\nEUR to VND: {:.3}\nUSD to VND: {:.3}\n\nGBP to PLN: {:.3}\nEUR to PLN: {:.3}",
        rates[&Currency::GBP][&Currency::VND],
        rates[&Currency::EUR][&Currency::VND],
        rates[&Currency::USD][&Currency::VND],
        rates[&Currency::GBP][&Currency::PLN],
        rates[&Currency::EUR][&Currency::PLN],
    );

    return message;
}
