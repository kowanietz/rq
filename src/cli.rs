use clap::Parser;

/// HTTP request CLI tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// URL to request
    pub(crate) url: String,

    /// Verbose mode
    #[arg(short, long, default_value_t = 0)]
    pub(crate) verbose: u8,
}
