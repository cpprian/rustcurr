use clap::Parser;

pub use crate::cli::structs::CliHandler;

pub fn process_cli_data<'a>() -> Result<CliHandler, anyhow::Error> {
    let cli = CliHandler::parse();

    Ok(cli)
}
