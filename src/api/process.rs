use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};

use crate::cli::CliHandler;

use super::{errors::ApiErrorResponse, structs::ApiResponse};

pub fn process_api_response(
    api_url: String,
    cli_data: CliHandler,
) -> Result<ApiResponse, ApiErrorResponse> {
    let url = format!("{}/latest/{}", api_url, cli_data.base);

    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header(AUTHORIZATION, "Bearer 12345")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .map_err(ApiErrorResponse::from)?;

    response.json().map_err(ApiErrorResponse::from)
}
