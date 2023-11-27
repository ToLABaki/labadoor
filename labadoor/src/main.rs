mod cli;
mod to_config;
use to_config::ToConfig;

macro_rules! add_cliargs {
    ($d:ident,$section:expr,$i:ident) => {
        $d.add_source(config::File::from_str(
            &format!("[{}]\n{}", $section, toml::to_string($i).unwrap()),
            config::FileFormat::Toml,
        ))
        .build()
        .unwrap()
    };
}

fn main() -> Result<(), ()> {
    let mut ret = Ok(());
    let cli = cli::parse();
    let config = config::Config::builder()
        .add_source(config::File::with_name(path).required(false))
        .add_source(config::Environment::with_prefix("LABADOOR").separator("_"));
    match &cli.command {
        #[cfg(feature = "telegram")]
        cli::Command::Telegram(cliargs) => {
            let config = add_cliargs!(config, "telegram", cliargs);
            let telegram = config.get::<cli::Telegram>("telegram").unwrap().to_config();
            labadoor_telegram::telegram(telegram);
        }
        #[cfg(feature = "matrix")]
        cli::Command::Matrix(cliargs) => {
            let config = add_cliargs!(config, "matrix", cliargs);
            let matrix = config.get::<cli::Matrix>("matrix").unwrap().to_config();
            labadoor_matrix::matrix(matrix);
        }
        #[cfg(feature = "gpio")]
        cli::Command::GPIO(cliargs) => {
            let config = add_cliargs!(config, "gpio", cliargs);
            let gpio = config.get::<cli::GPIO>("gpio").unwrap().to_config();
            labadoor_gpio::gpio(gpio);
        }
        #[cfg(feature = "open")]
        cli::Command::Open(cliargs) => {
            let config = add_cliargs!(config, "open", cliargs);
            let open = config.get::<cli::Open>("open").unwrap().to_config();
            labadoor_open::open(open);
        }
        #[cfg(feature = "auth")]
        cli::Command::Auth(cli) => ret = labadoor_auth::auth(&cli, config),
    }
    ret
}
