use crate::cli::CliHandler;

use super::{errors::ApiErrorResponse, structs::ApiResponse};

pub fn process_api_response(
    api_url: String,
    cli_data: CliHandler,
) -> Result<ApiResponse, ApiErrorResponse> {
    let url = format!("{}{}&symbols={}", api_url, cli_data.base, cli_data.target);
    let response = reqwest::blocking::get(url)
        .unwrap()
        .json::<ApiResponse>()
        .unwrap_or_else(|error| {
            let error_message = format!("Error: {}", error);
            let api_error_response = ApiErrorResponse {
                result: "error".to_string(),
                error_type: error_message,
            };
            panic!("{:?}", api_error_response);
        });

    Ok(response)
}
