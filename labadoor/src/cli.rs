use clap::Parser;
use serde::{Deserialize, Serialize};

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
    #[cfg(feature = "gpio")]
    GPIO(GPIO),
    #[cfg(feature = "auth")]
    Auth(labadoor_auth::cli::Cli),
    #[cfg(feature = "open")]
    Open(Open),
}

#[cfg(feature = "telegram")]
#[derive(Serialize, Deserialize, Parser, Debug)]
pub struct Telegram {
    #[clap(skip)]
    pub trigger: Option<Vec<String>>,
    #[clap(short, long)]
    pub token: Option<String>,
}

#[cfg(feature = "matrix")]
#[derive(Serialize, Deserialize, Parser, Debug)]
pub struct Matrix {
    #[clap(short, long)]
    pub username: Option<String>,
    #[clap(short, long)]
    pub password: Option<String>,
    #[clap(short, long)]
    pub device_id: Option<String>,
}

#[cfg(feature = "gpio")]
#[derive(Serialize, Deserialize, Parser, Debug)]
pub struct GPIO {
    #[clap(short, long)]
    pub device: Option<String>,
    #[clap(short, long)]
    pub pin: Option<u8>,
    #[clap(short, long)]
    #[arg(default_value = "Some(false)")]
    pub active_low: Option<bool>,
    #[clap(short = 't', long)]
    #[arg(default_value = "Some(2000)")]
    pub active_time: Option<u32>,
}

use std::collections::BTreeMap;
#[cfg(feature = "open")]
#[derive(Serialize, Deserialize, Parser, Clone, Debug)]
pub struct Open {
    #[clap(skip)]
    pub auth: BTreeMap<String, Vec<String>>,
    #[clap(skip)]
    pub hardware: BTreeMap<String, Vec<String>>,
    pub method: String,
    pub identifier: String,
    pub resource_shortcut: i8,
}

pub fn parse() -> Cli {
    Cli::parse()
}
