//! Command line arguments parsed using clap

use clap::Parser;

/// Comand line argments
#[derive(Parser)]
#[command(version)]
pub(crate) struct Args {
    /// Currency to convert from
    #[arg(value_name = "basecode")]
    pub(crate) base: String,


    /// Currency to convert to
    #[arg(value_name = "targetcode", default_value_t = String::new())]
    pub(crate) target: String,

    /// Units to convert
    #[arg(value_name = "units", default_value_t = 1.00)]
    pub(crate) units: f64,
}

impl Args {
    /// Get parsed arguments from the command line
    pub(crate) fn new() -> Self {
        Self::parse()
    }
}
