use clap::Parser;

#[derive(Parser)]
#[command(version)]
pub(crate) struct Args {
    /// Currency to convert from
    #[arg(short, long, value_name = "BASECODE")]
    pub(crate) base: String,


    /// Currency to convert to
    #[arg(short, long, value_name = "TARGETCODE", default_value_t = String::new())]
    pub(crate) target: String,

    /// Units to convert
    #[arg(short, long, default_value_t = 1.00)]
    pub(crate) units: f64,
}

impl Args {
    pub(crate) fn new() -> Self {
        Self::parse()
    }
}