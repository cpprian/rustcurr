use std::{error::Error, fmt::Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub result: String,
    pub error_type: String,
}

impl Display for ApiErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.error_type)
    }
}

impl Error for ApiErrorResponse {}

impl From<reqwest::Error> for ApiErrorResponse {
    fn from(error: reqwest::Error) -> Self {
        ApiErrorResponse {
            result: "error".to_string(),
            error_type: error.to_string(),
        }
    }
}

// the trait bound `ApiErrorResponse: std::error::Error` is not satisfied
// the following other types implement trait `FromResidual<R>`:
// <Result<T, F> as FromResidual<Yeet<E>>>
// <Result<T, F> as FromResidual<Result<Infallible, E>>>
// required for `anyhow::Error` to implement `From<ApiErrorResponse>`
// required for `Result<(), anyhow::Error>` to implement `FromResidual<Result<Infallible, ApiErrorResponse>>`