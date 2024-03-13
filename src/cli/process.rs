use clap::Parser;

pub use crate::cli::structs::CliHandler;

pub fn process_cli_data<'a>(writer: &mut impl std::io::Write) -> Result<CliHandler, anyhow::Error> {
    let cli = CliHandler::parse();

    // TODO: consider add this message as a log
    writeln!(writer, "CLI data loaded successfully")?;

    Ok(cli)
}

// TODO: add tests
