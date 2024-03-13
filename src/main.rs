use anyhow::{Context, Result};
use api::process;
use rustcurr::{
    api,
    converter::{self, structs::CurrencyConversion},
};

mod setup;
mod structs;

fn main() -> Result<()> {
    run().context("Error running the app")?;
    Ok(())
}

fn run() -> Result<()> {
    // setup application
    let app = setup::setup_app().context("Error setting up the app")?;
    let response = process::process_api_response(app.api_url, &app.user_data)?;

    // convert logic
    let conversion = converter::process_conversion(
        response,
        app.user_data.base.as_str(),
        app.user_data.target.as_str(),
        app.user_data.amount,
    )?;

    // print result
    print_result(conversion, &mut std::io::stdout())?;

    Ok(())
}

fn print_result(conversion: CurrencyConversion, writer: &mut impl std::io::Write) -> Result<()> {
    writeln!(writer, "\n----- Conversion Result -----")?;
    writeln!(writer, "{}", conversion)?;
    Ok(())
}
