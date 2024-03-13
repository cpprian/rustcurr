use rustcurr::{cli::structs as cli, config::structs as config};

/// Core application struct that holds all the data from user and API url for ExchangeRate Api communication
pub struct App {
    /// User data from CLI it holds base currency, target currency and amount
    pub user_data: cli::CliHandler,
    
    /// API url for ExchangeRate Api
    pub api_url: String,
}
