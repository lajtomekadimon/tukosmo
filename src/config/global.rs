use serde::Deserialize;
use std::fs;
use toml;


pub const SUPPORTED_LANGUAGES: &'static [&'static str] = &[
    "en",  // English
    "es",  // Spanish (Español)
];


#[derive(Clone, Deserialize)]
pub struct PreConfig {
    pub database: ConfigDatabase,
    pub server: ConfigServer,
}

#[derive(Clone, Deserialize)]
pub struct ConfigDatabase {
    pub host: String,
    pub name: String,
    pub user: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct ConfigServer {
    pub mode: String,
    pub default_lang: String,
    pub theme: String,
    pub domain: String,
    pub development: ConfigServerDevelopment,
    pub production: ConfigServerProduction,
}

#[derive(Clone, Deserialize)]
pub struct ConfigServerDevelopment {
    pub http_port: String,
    pub https_port: String,
}

#[derive(Clone, Deserialize)]
pub struct ConfigServerProduction {
    pub http_port: String,
    pub https_port: String,
}

#[derive(Clone, Deserialize)]
pub struct Config {
    pub dbauth: String,
    pub server: ConfigServer,
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
        dbauth: db_auth_string.to_string(),
        server: config.server,
    }
}

