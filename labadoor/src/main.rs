mod cli;

fn main() {
    let cli = cli::parse();
    match &cli.command {
        #[cfg(feature = "telegram")]
        cli::Command::Telegram(_) => {
            labadoor_telegram::telegram();
        }
        #[cfg(feature = "matrix")]
        cli::Command::Matrix(_) => {
            labadoor_matrix::matrix();
        }
        #[cfg(feature = "csv")]
        cli::Command::CSV => {
            labadoor_csv::csv();
        }
        #[cfg(feature = "gpio")]
        cli::Command::GPIO => {
            labadoor_gpio::gpio();
        }
    }
}
