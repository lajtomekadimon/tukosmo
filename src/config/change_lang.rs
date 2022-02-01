use toml;
use std::fs;

use crate::config::global::{
    Config,
    PreConfig,
    ConfigServer,
};


pub fn change_lang(
    config: &Config,
    default_lang_value: &str,
) {
    let new_toml_file = toml::to_string(
        &PreConfig {
            server: ConfigServer {
                mode: (&config.server.mode).clone(),
                domain: (&config.server.domain).clone(),
                user_email: (&config.server.user_email).clone(),
                reset: (&config.server.reset).clone(),
                default_lang: default_lang_value.to_string(),
                theme: (&config.server.theme).clone(),
                development: (&config.server.development).clone(),
                production: (&config.server.production).clone(),
            },
            database: (&config.database).clone(),
        }
    ).unwrap();
    fs::write("Tukosmo.toml", new_toml_file).unwrap();
}