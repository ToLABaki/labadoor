pub mod cli;
pub mod to_config;
use labadoor_acl::ACL;
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

use config::{builder::DefaultState, ConfigBuilder};
pub fn auth(cli: &cli::Cli, config: ConfigBuilder<DefaultState>) -> Result<(), ()> {
    let mut ret = Err(());
    let config = add_cliargs!(config, "auth", cli);
    let backends = config.get::<Vec<cli::Backend>>("auth.backends").unwrap();
    for backend in backends {
        let acl = match backend {
            #[cfg(feature = "csv")]
            cli::Backend::CSV => config.get::<cli::CSV>("csv").unwrap().to_config().new(),
        };
        match &cli.command {
            cli::Command::Open(cliargs) => {
                let found = acl.auth_user(
                    cliargs.method.clone(),
                    cliargs.identifier.clone(),
                    cliargs.resource.clone(),
                );
                if let Some(one) = found {
                    println!("{},{}", one.username, one.resource);
                    ret = Ok(())
                }
            }
        };
    }
    ret
}
