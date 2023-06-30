use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Telegram,
    Matrix,
    CSV,
    GPIO,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Command::Telegram => {
            labadoor_telegram::main();
        }
        Command::Matrix => {
            labadoor_matrix::main();
        }
        Command::CSV => {
            labadoor_csv::main();
        }
        Command::GPIO => {
            labadoor_gpio::main();
        }
    }
}
