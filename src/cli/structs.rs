/// Struct to hold the CLI arguments
#[derive(Clone, Debug)]
pub struct CliHandler<'a> {
    /// Base currency code
    pub base: &'a str,

    /// Target currency code
    pub target: &'a str,

    /// Amount to convert
    pub amount: f64,
}
