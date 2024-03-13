use anyhow::{Context, Result};
use std::io::{self, Write};

mod config;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let config = config::process_config_data(&mut writer)?;

    writeln!(writer, "API_KEY: {}", config.api_key).context("Failed to write to stdout")?;
    Ok(())
}
