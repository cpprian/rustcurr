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
    if app.user_data.list {
        print_list_of_currencies(response);
        return Ok(());
    }
    let conversion = converter::process_conversion(
        response,
        app.user_data.base.as_str(),
        app.user_data.target.as_str(),
        app.user_data.amount,
    )?;

    // print result
    print_result_conversion(conversion, &mut std::io::stdout())?;

    Ok(())
}

fn print_result_conversion(
    conversion: CurrencyConversion,
    writer: &mut impl std::io::Write,
) -> Result<()> {
    writeln!(writer, "\n----- Conversion Result -----")?;
    writeln!(writer, "{}", conversion)?;
    Ok(())
}

fn print_list_of_currencies(response: api::structs::ApiResponse) {
    for (key, value) in response.conversion_rates.iter() {
        println!("{}: {}", key, value);
    }
}
