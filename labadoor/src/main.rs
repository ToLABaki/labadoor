use labadoor_acl::ACL;
mod cli;
mod to_config;
use to_config::ToConfig;

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
            let telegram = config.get::<cli::Telegram>("telegram").unwrap().to_config();
            labadoor_telegram::telegram(telegram);
        }
        #[cfg(feature = "matrix")]
        cli::Command::Matrix(_) => {
            let matrix = config.get::<cli::Matrix>("matrix").unwrap().to_config();
            labadoor_matrix::matrix(matrix);
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
            let gpio = config.get::<cli::GPIO>("gpio").unwrap().to_config();
            labadoor_gpio::gpio(gpio);
        }
        #[cfg(feature = "auth")]
        cli::Command::Auth(cli) => {
            labadoor_auth::auth(cli);
        }
    }
}
