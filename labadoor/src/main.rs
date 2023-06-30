mod cli;

fn main() {
    let cli = cli::parse();
    match &cli.command {
        #[cfg(feature = "telegram")]
        cli::Command::Telegram(_) => {
            labadoor_telegram::main();
        }
        #[cfg(feature = "matrix")]
        cli::Command::Matrix(_) => {
            labadoor_matrix::main();
        }
        #[cfg(feature = "csv")]
        cli::Command::CSV => {
            labadoor_csv::main();
        }
        #[cfg(feature = "gpio")]
        cli::Command::GPIO => {
            labadoor_gpio::main();
        }
    }
}
