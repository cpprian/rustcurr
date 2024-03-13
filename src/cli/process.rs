pub use crate::cli::structs::CliHandler;

pub fn process_cli_data() -> Result<CliHandler, anyhow::Error> {
    let data = CliHandler {
        base: "".to_string(),
        target: "".to_string(),
        amount: 0.0,
    };

    Ok(data)
}
