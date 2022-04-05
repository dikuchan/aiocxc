use std::collections::BTreeMap;
use serde::{Deserialize, Deserializer};

/// Converts chrono datetime to timestamp in milliseconds.
pub mod datetime_to_timestamp {
    use pyo3_chrono::{NaiveDateTime};
    use serde::{Serializer, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
        where D: Deserializer<'de>
    {
        let timestamp = i64::deserialize(deserializer)?;
        let seconds = timestamp / 1000;
        let nanos = timestamp % 1000;
        let datetime = chrono::NaiveDateTime::from_timestamp(seconds, nanos as u32);
        let datetime = NaiveDateTime::from(datetime);

        Ok(datetime)
    }

    pub fn serialize<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let timestamp = datetime.0.timestamp_millis();
        serializer.serialize_i64(timestamp)
    }
}

type Orders = BTreeMap<String, String>;

pub fn to_orderbook<'de, D>(deserializer: D) -> Result<Orders, D::Error>
    where D: Deserializer<'de>
{
    let mut mapping = BTreeMap::new();
    let orders: Vec<Vec<_>> = Deserialize::deserialize(deserializer)?;
    for mut order in orders.into_iter() {
        let price = order.remove(0);
        let amount = order.remove(0);
        mapping.insert(price, amount);
    }

    Ok(mapping)
}