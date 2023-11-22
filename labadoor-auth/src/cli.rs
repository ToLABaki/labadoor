use clap::{Parser, ValueEnum};
use serde::Deserialize;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, value_enum)]
    pub backend: Backend,
}

#[derive(Parser, ValueEnum, Clone, Debug)]
pub enum Backend {
    #[cfg(feature = "csv")]
    CSV,
}

#[derive(Parser, Debug)]
pub enum Command {
    Open(Open),
}

#[derive(Parser, Debug)]
pub struct Open {
    pub method: String,
    pub identifier: String,
    pub resource: i8,
}

pub fn parse() -> Cli {
    Cli::parse()
}

#[cfg(feature = "csv")]
#[derive(Deserialize, Parser, Debug)]
pub struct CSV {
    #[clap(short, long)]
    #[arg(default_value = "Some(String::from(\"/etc/labadoor\"))")]
    pub path: String,
}
