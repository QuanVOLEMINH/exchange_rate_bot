use serde::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Deserialize, Debug)]
pub struct ExchangeRateResponse {
    pub result: String,
    pub base_code: Currency,
    #[serde(deserialize_with = "deserialize_conversion_rates")]
    pub conversion_rates: HashMap<Currency, f64>,
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum Currency {
    GBP,
    EUR,
    PLN,
    USD,
    VND,
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Currency::GBP => "GBP",
                Currency::EUR => "EUR",
                Currency::PLN => "PLN",
                Currency::USD => "USD",
                Currency::VND => "VND",
            }
        )
    }
}

impl TryFrom<&str> for Currency {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GBP" => Ok(Currency::GBP),
            "EUR" => Ok(Currency::EUR),
            "PLN" => Ok(Currency::PLN),
            "USD" => Ok(Currency::USD),
            "VND" => Ok(Currency::VND),
            _ => Err(()),
        }
    }
}

fn deserialize_conversion_rates<'de, D>(deserializer: D) -> Result<HashMap<Currency, f64>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: HashMap<String, f64> = HashMap::deserialize(deserializer)?;
    let filtered_map = map
        .into_iter()
        .filter_map(|(k, v)| Currency::try_from(k.as_str()).ok().map(|k| (k, v)))
        .collect();

    Ok(filtered_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_exchange_rate_response_deserialization() {
        let json = r#"
        {
            "result": "success",
            "base_code": "GBP",
            "conversion_rates": {
                "EUR": 1.1,
                "USD": 1.2,
                "Unknown": 0.0
            }
        }
        "#;

        let response: ExchangeRateResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.result, "success");
        assert!(response.conversion_rates.contains_key(&Currency::EUR));
        assert!(response.conversion_rates.contains_key(&Currency::USD));
    }
}
