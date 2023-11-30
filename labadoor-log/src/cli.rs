use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short, long, value_enum)]
    pub backends: Option<Vec<Backend>>,
}

#[derive(Serialize, Deserialize, Parser, ValueEnum, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Backend {
    #[cfg(feature = "csv")]
    CSV,
}

#[derive(Serialize, Parser, Debug)]
pub enum Command {
    Append(Append),
}

#[derive(Serialize, Parser, Debug)]
pub struct Append {
    pub time: String,
    pub username: String,
    pub resource: String,
    pub method: String,
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
