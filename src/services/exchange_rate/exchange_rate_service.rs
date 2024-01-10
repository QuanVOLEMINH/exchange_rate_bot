use super::exchange_rate_response::{Currency, ExchangeRateResponse};

pub struct ExchangeRateService {
    api_url: String,
}

impl ExchangeRateService {
    pub fn new(api_base_url: String, api_key: String) -> Self {
        let api_url = format!("{}/{}", api_base_url, api_key);
        ExchangeRateService { api_url }
    }

    pub async fn get_exchange_rate(
        &self,
        symbol: &Currency,
    ) -> Result<ExchangeRateResponse, reqwest::Error> {
        let latest_rate_url = format!("{}/latest/{}", &self.api_url, symbol);
        let response = reqwest::Client::new()
            .get(&latest_rate_url)
            .send()
            .await?
            .json::<ExchangeRateResponse>()
            .await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_get_exchange_rate_success() {
        let api_base_url = server_url();
        let api_key = "random_key";
        let test_symbol = Currency::GBP;
        let api_url = format!("/{}/latest/{}", api_key, test_symbol);

        let _m = mock("GET", api_url.as_str())
            .with_status(200)
            .with_body(
                serde_json::json!({
                    "result": "success",
                    "base_code": "GBP",
                    "conversion_rates": {
                        "EUR": 1.1,
                        "USD": 1.2
                    }
                })
                .to_string(),
            )
            .create();
        let service = ExchangeRateService::new(api_base_url, api_key.to_string());
        let result: Result<ExchangeRateResponse, reqwest::Error> =
            service.get_exchange_rate(&test_symbol).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.base_code, test_symbol);
        assert!(response.conversion_rates.contains_key(&Currency::EUR));
        assert_eq!(response.conversion_rates.get(&Currency::EUR), Some(&1.1f64));
    }
}
