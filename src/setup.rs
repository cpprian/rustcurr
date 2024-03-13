use anyhow::{Error, Result};
use rustcurr::{cli, config};
use std::io;

use crate::structs::App;

/// Setup the application by processing the config and CLI data from the user
pub fn setup_app() -> Result<App, Error> {
    let config = config::process_config_data(&mut io::stdout().lock())?;
    let user_data = cli::process_cli_data()?;
    let app = App {
        user_data,
        api_url: format!("https://v6.exchangerate-api.com/v6/{}", config.api_key),
    };

    Ok(app)
}
