use anyhow::{Error, Result};
use rustcurr::{cli, config};
use std::io;

use crate::structs::App;

pub fn setup_app() -> Result<App, Error> {
    let config = config::process_config_data(&mut io::stdout().lock())?;
    let user_data = cli::process_cli_data()?;
    let app = App {
        config,
        user_data,
        api_url: "https://v6.exchangeratesapi.io/latest?base={}&symbols={}".to_string(),
    };

    Ok(app)
}
