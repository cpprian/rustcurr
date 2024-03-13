/// Struct to hold the CLI arguments
pub struct CliHandler {
    /// Base currency code
    pub base: String,

    /// Target currency code
    pub target: String,

    /// Amount to convert
    pub amount: f64,
}
