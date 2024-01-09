use serde::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
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
