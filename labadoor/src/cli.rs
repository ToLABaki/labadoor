use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[cfg(feature = "telegram")]
    Telegram(Telegram),
    #[cfg(feature = "matrix")]
    Matrix(Matrix),
    #[cfg(feature = "csv")]
    CSV(CSV),
    #[cfg(feature = "gpio")]
    GPIO(GPIO),
}

#[cfg(feature = "telegram")]
#[derive(Deserialize, Parser, Debug)]
pub struct Telegram {
    #[clap(short, long)]
    pub token: Option<String>,
}

#[cfg(feature = "matrix")]
#[derive(Deserialize, Parser, Debug)]
pub struct Matrix {
    #[clap(short, long)]
    pub username: Option<String>,
    #[clap(short, long)]
    pub password: Option<String>,
}

#[cfg(feature = "csv")]
#[derive(Deserialize, Parser, Debug)]
pub struct CSV {
    #[clap(short, long)]
    #[arg(default_value = "Some(String::from(\"/etc/labadoor\"))")]
    pub path: Option<String>,
    #[clap(short, long)]
    pub method: String,
    #[clap(short, long)]
    pub identifier: String,
    // #[cfg(feature = "multiple_resources")]
    #[clap(short, long)]
    pub resource_shortcut: i8,
}

#[cfg(feature = "gpio")]
#[derive(Deserialize, Parser, Debug)]
pub struct GPIO {
    #[clap(short, long)]
    pub device: Option<String>,
    #[clap(short, long)]
    pub pin: Option<u8>,
    #[clap(short, long)]
    pub active_low: Option<bool>,
    #[clap(short = 't', long)]
    pub active_time: Option<u32>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
