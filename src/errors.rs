use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;

pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to send an HTTP request")]
    Request(#[source] reqwest::Error),
    #[error("Failed to deserialize an HTTP response")]
    Deserialize(#[source] reqwest::Error),
    #[error("Failed to access an API {code:?}: {message:?}")]
    API { code: i16, message: String },
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        match err {
            Error::Request(err) => PyRuntimeError::new_err(err.to_string()),
            Error::Deserialize(err) => PyValueError::new_err(err.to_string()),
            Error::API { code, message } => PyRuntimeError::new_err(format!("{}: {}", code, message))
        }
    }
}