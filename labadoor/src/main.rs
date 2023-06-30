use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    #[cfg(feature = "telegram")]
    Telegram,
    #[cfg(feature = "matrix")]
    Matrix,
    #[cfg(feature = "csv")]
    CSV,
    #[cfg(feature = "gpio")]
    GPIO,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        #[cfg(feature = "telegram")]
        Command::Telegram => {
            labadoor_telegram::main();
        }
        #[cfg(feature = "matrix")]
        Command::Matrix => {
            labadoor_matrix::main();
        }
        #[cfg(feature = "csv")]
        Command::CSV => {
            labadoor_csv::main();
        }
        #[cfg(feature = "gpio")]
        Command::GPIO => {
            labadoor_gpio::main();
        }
    }
}
