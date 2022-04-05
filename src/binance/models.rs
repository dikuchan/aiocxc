use pyo3::prelude::*;
use pyo3_chrono::NaiveDateTime;

use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};
use serde::{Deserialize, Deserializer};
use crate::utils::*;

#[derive(Debug)]
struct Symbol {
    base: String,
    quote: String,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}/{}", self.base, self.quote)
    }
}

// TODO: Actually split symbol.
impl<'de> Deserialize<'de> for Symbol {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        Ok(Self { base: "".to_owned(), quote: "".to_owned() })
    }
}

#[pyclass]
#[derive(Deserialize, Debug)]
pub struct Time {
    #[pyo3(get)]
    #[serde(rename = "serverTime", with = "datetime_to_timestamp")]
    time: NaiveDateTime,
}

// TODO: Use decimal.
#[pyclass]
#[derive(Deserialize, Debug)]
pub struct OrderBook {
    #[pyo3(get)]
    #[serde(rename = "lastUpdateId")]
    nonce: u64,
    #[pyo3(get)]
    #[serde(deserialize_with = "to_orderbook")]
    asks: BTreeMap<String, String>,
    #[pyo3(get)]
    #[serde(deserialize_with = "to_orderbook")]
    bids: BTreeMap<String, String>,
}
