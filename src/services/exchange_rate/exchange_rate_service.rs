use super::exchange_rate_response::ExchangeRateResponse;

pub struct ExchangeRateService {
    api_url: String,
}

impl ExchangeRateService {
    const BASE_URL: &'static str = "https://v6.exchangerate-api.com/v6";

    pub fn new(api_key: String) -> Self {
        let api_url = format!("{}/{}", Self::BASE_URL, api_key);
        ExchangeRateService { api_url }
    }

    pub async fn get_exchange_rate(
        &self,
        symbol: &str,
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
