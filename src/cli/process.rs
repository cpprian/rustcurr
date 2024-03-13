pub use crate::cli::structs::CliHandler;

pub fn process_cli_data<'a>() -> Result<CliHandler<'a>, anyhow::Error> {
    let data = CliHandler {
        base: "USD",
        target: "PLN",
        amount: 10.0,
    };

    Ok(data)
}
