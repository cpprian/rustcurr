/// Struct to hold the configuration data
///
/// # Fields
///
/// * `api_key` - The API key
#[derive(Debug)]
pub struct ConfigData {
    /// The API key to work with the ExchangeRate API
    pub api_key: String,
}

impl ConfigData {
    /// Create a new instance of the ConfigData struct
    pub fn new(api_key: String) -> ConfigData {
        ConfigData { api_key }
    }
}
