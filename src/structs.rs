use rustcurr::{cli::structs as cli, config::structs as config};

pub struct App {
    pub config: config::ConfigData,
    pub user_data: cli::CliHandler,
    pub api_url: String,
}
