use anyhow::{Context, Result};
use dotenv::dotenv;

use super::structs::ConfigData;

/// Process the config data from environment variables
///
/// # Arguments
///
/// * `writer` - The writer to write the output to the console
///
/// # Returns
///
/// The config data from the environment variables as a ConfigData struct
pub fn process_config_data(writer: &mut impl std::io::Write) -> Result<ConfigData> {
    dotenv().context("Failed to read .env file")?;
    let config = load_config_data()?;
    // TODO: change to log
    writeln!(writer, "API_KEY: {}", config.api_key).unwrap();
    Ok(config)
}

/// Load the config data to the ConfigData struct
///
/// # Returns
///
/// The config data as a ConfigData struct
fn load_config_data() -> Result<ConfigData> {
    let config = std::env::var("API_KEY").with_context(|| "API_KEY not found in config file")?;
    Ok(ConfigData::new(config))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_data() {
        std::env::set_var("API_KEY", "api_key");
        let config = load_config_data().unwrap();
        assert_eq!(config.api_key, "api_key");
    }

    #[test]
    fn test_process_config_data() {
        std::env::set_var("API_KEY", "api_key");
        let mut writer = Vec::new();
        let config = process_config_data(&mut writer).unwrap();
        assert_eq!(config.api_key, "api_key");
    }
}
