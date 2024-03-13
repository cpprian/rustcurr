use anyhow::{Context, Result};
use rustcurr::api;
use api::process;

mod setup;
mod structs;

fn main() -> Result<()> {
    run().context("Error running the app")?;
    Ok(())
}

fn run() -> Result<()> {
    let app = setup::setup_app().context("Error setting up the app")?;
    // let response = process::process_api_response(app.api_url, app.user_data)?;
    // println!("{}", response);

    // convert logic

    Ok(())
}