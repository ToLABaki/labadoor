mod cli;

fn main() {
    let cli = cli::parse();
    let config = config::Config::builder()
        .add_source(config::File::with_name("./config.toml").required(false))
        .add_source(config::Environment::with_prefix("LABADOOR").separator("_"))
        .build()
        .unwrap();
    match &cli.command {
        #[cfg(feature = "telegram")]
        cli::Command::Telegram(_) => {
            let telegram = config.get::<cli::Telegram>("telegram").unwrap();
            labadoor_telegram::telegram(telegram.token.unwrap());
        }
        #[cfg(feature = "matrix")]
        cli::Command::Matrix(_) => {
            let matrix = config.get::<cli::Matrix>("matrix").unwrap();
            labadoor_matrix::matrix(matrix.username.unwrap(), matrix.password.unwrap());
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
