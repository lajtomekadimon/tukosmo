use serde::{Deserialize, Serialize};
use std::fs;
use toml;


pub const SUPPORTED_LANGUAGES: &'static [&'static str] = &[
    "en",  // English
    "es",  // Spanish (Español)
];


#[derive(Clone, Deserialize, Serialize)]
pub struct PreConfig {
    pub server: ConfigServer,
    pub database: ConfigDatabase,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigServer {
    pub mode: String,
    pub domain: String,
    pub reset: String,
    pub default_lang: String,
    pub theme: String,
    pub development: ConfigServerDevelopment,
    pub production: ConfigServerProduction,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigServerDevelopment {
    pub http_port: String,
    pub https_port: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigServerProduction {
    pub http_port: String,
    pub https_port: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigDatabase {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ConfigServer,
    pub database: ConfigDatabase,
    pub dbauth: String,
}


pub fn config() -> Config {
    let toml_file = &fs::read_to_string("Tukosmo.toml")
        .expect("Something went wrong reading Tukosmo.toml!");
    let config: PreConfig = toml::from_str(toml_file).unwrap();

    let db_auth_string = &format!(
        "host={} dbname={} user={} password={}",
        config.database.host,
        config.database.name,
        config.database.user,
        config.database.password,
    );

    Config {
        server: config.server,
        database: config.database,
        dbauth: db_auth_string.to_string(),
    }
}

