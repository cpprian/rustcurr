pub use crate::cli::structs::CliHandler;

pub fn process_cli_data() -> Result<CliHandler, anyhow::Error> {
    let data = CliHandler {
        base: "USD".to_string(),
        target: "PLN".to_string(),
        amount: 10.0,
    };

    Ok(data)
}
