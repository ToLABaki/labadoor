pub mod cli;
pub mod to_config;
use labadoor_acl::ACL;
use to_config::ToConfig;

pub fn auth(i: &cli::Cli, config: config::Config) {
    let acl = match &i.backend {
        #[cfg(feature = "csv")]
        cli::Backend::CSV => config.get::<cli::CSV>("csv").unwrap().to_config().new(),
    };
    match &i.command {
        cli::Command::Open(open) => acl.auth_user(
            open.method.clone(),
            open.identifier.clone(),
            open.resource.clone(),
        ),
    };
}
