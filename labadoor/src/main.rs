use labadoor_acl::ACL;
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
            labadoor_matrix::matrix(
                matrix.username.unwrap(),
                matrix.password.unwrap(),
                matrix.device_id,
            );
        }
        #[cfg(feature = "csv")]
        cli::Command::CSV(_) => {
            let csv = config.get::<cli::CSV>("csv").unwrap();
            let c = labadoor_csv::CSV {
                path: csv.path.unwrap(),
            };
            c.auth_user(csv.method, csv.identifier, csv.resource_shortcut);
        }
        #[cfg(feature = "gpio")]
        cli::Command::GPIO(_) => {
            let gpio = config.get::<cli::GPIO>("gpio").unwrap();
            labadoor_gpio::gpio(
                gpio.device.unwrap(),
                gpio.pin.unwrap(),
                gpio.active_low.unwrap(),
                gpio.active_time.unwrap(),
            );
        }
    }
}
