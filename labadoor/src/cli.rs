use clap::Parser;

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
    CSV,
    #[cfg(feature = "gpio")]
    GPIO,
}

#[cfg(feature = "telegram")]
#[derive(Parser, Debug)]
pub struct Telegram {
    #[clap(short, long)]
    pub token: Option<String>,
}

#[cfg(feature = "matrix")]
#[derive(Parser, Debug)]
pub struct Matrix {
    #[clap(short, long)]
    pub username: Option<String>,
    #[clap(short, long)]
    pub password: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
