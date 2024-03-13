use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub result: String,
    pub error_type: String,
}


// the trait bound `ApiErrorResponse: std::error::Error` is not satisfied
// the following other types implement trait `FromResidual<R>`:
// <Result<T, F> as FromResidual<Yeet<E>>>
// <Result<T, F> as FromResidual<Result<Infallible, E>>>
// required for `anyhow::Error` to implement `From<ApiErrorResponse>`
// required for `Result<(), anyhow::Error>` to implement `FromResidual<Result<Infallible, ApiErrorResponse>>`