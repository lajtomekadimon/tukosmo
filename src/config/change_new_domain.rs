use toml;
use std::fs;

use crate::config::global::{
    Config,
    PreConfig,
    ConfigServer,
};


pub fn change_new_domain(
    config: &Config,
    new_domain_value: &str,
) {
    let new_toml_file = toml::to_string(
        &PreConfig {
            server: ConfigServer {
                server_os: (&config.server.server_os).clone(),
                mode: (&config.server.mode).clone(),
                domain: (&config.server.domain).clone(),
                new_domain: new_domain_value.to_string(),
                user_email: (&config.server.user_email).clone(),
                reset: (&config.server.reset).clone(),
                default_lang: (&config.server.default_lang).clone(),
                theme: (&config.server.theme).clone(),
                development: (&config.server.development).clone(),
                production: (&config.server.production).clone(),
            },
            database: (&config.database).clone(),
            modules: (&config.modules).clone(),
        }
    ).unwrap();
    fs::write("etc/Tukosmo.toml", new_toml_file).unwrap();
}
