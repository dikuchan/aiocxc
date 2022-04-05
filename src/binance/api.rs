use pyo3::prelude::*;

use std::{
    str::FromStr,
    sync::atomic::AtomicU16,
};
use serde::{de::DeserializeOwned, Deserialize};
use reqwest::{
    header::{HeaderValue, HeaderMap},
    Client, Request, Method, Url,
};
use super::models::*;

/// Client is an Arc internally, so clone before sending a request.
///
async fn request<T>(
    client: Client,
    method: Method,
    endpoint: &str,
    params: Option<Vec<(&str, String)>>,
) -> crate::Result<T>
    where T: DeserializeOwned + Send + 'static
{
    let url = format!("https://api.binance.com/api/v3/{}", endpoint);
    // Infallible.
    let mut url = Url::from_str(&url).expect("Failed to build an URL");
    if let Some(params) = params {
        let mut query = url.query_pairs_mut();
        for (name, value) in params.into_iter() {
            query.append_pair(name, &value);
        }
    }
    let request = Request::new(method, url);
    let response = client
        .execute(request)
        .await
        .map_err(|err| crate::Error::Request(err))?
        .json::<Response<T>>()
        .await
        .map_err(|err| crate::Error::Deserialize(err))?
        .body()?;

    Ok(response)
}

#[pyclass]
#[derive(Deserialize)]
struct Error {
    code: i16,
    #[serde(rename = "msg")]
    message: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Body(T),
    Err(Error),
}

impl<T> Response<T> {
    fn body(self) -> crate::Result<T> {
        match self {
            Self::Body(body) => Ok(body),
            Self::Err(Error { code, message }) => Err(crate::Error::API { code, message }),
        }
    }
}


#[pyclass]
pub struct Binance {
    secret: Vec<u8>,
    client: Client,
    /// Receive time window.
    window: u16,
    /// Stores used request weight.
    weight: AtomicU16,
}

#[pymethods]
impl Binance {
    #[new]
    pub fn new(key: String, secret: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&key)
            .expect("Failed to initialize an HTTP client"),
        );
        let secret = secret.into_bytes();
        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to initialize an HTTP client");

        Self { secret, client, window: 5000, weight: AtomicU16::new(0) }
    }

    pub fn time<'p>(&self, py: Python<'p>) -> PyResult<&'p PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let response: Time = request(client, Method::GET, "time", None)
                .await?;

            Ok(response)
        })
    }

    pub fn orderbook<'p>(&self, py: Python<'p>, symbol: String, limit: u16) -> PyResult<&'p PyAny> {
        let client = self.client.clone();
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let params = vec![("symbol", symbol), ("limit", limit.to_string())];
            let response: OrderBook = request(client, Method::GET, "depth", Some(params))
                .await?;

            Ok(response)
        })
    }
}
