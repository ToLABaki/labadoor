pub mod cli;
mod to_config;
use crate::to_config::ToConfig;
use labadoor_logging::Logging;

pub struct LogArgs {
    pub username: String,
    pub resource: String,
    pub method: String,
}
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

use config::{builder::DefaultState, ConfigBuilder};
pub fn log(cli: &cli::Cli, config: ConfigBuilder<DefaultState>) -> Result<(), ()> {
    let mut ret = Err(());
    let config = add_cliargs!(config, "log", cli);
    let backends = config.get::<Vec<cli::Backend>>("log.backends").unwrap();
    for backend in backends {
        let logging = match backend {
            #[cfg(feature = "csv")]
            cli::Backend::CSV => config.get::<cli::CSV>("log.csv").unwrap().to_config().new(),
        };
        match &cli.command {
            cli::Command::Append(cliargs) => {
                logging.append(
                    "time.new()".to_string(),
                    cliargs.username.clone(),
                    cliargs.resource.clone(),
                    cliargs.method.clone(),
                );
                ret = Ok(());
            }
        };
    }
    ret
}
