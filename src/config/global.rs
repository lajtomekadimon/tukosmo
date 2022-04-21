use serde::{Deserialize, Serialize};
use std::fs;
use toml;


pub const TUKOSMO_VERSION: &'static str = "0.1";


pub const SUPPORTED_LANGUAGES: &'static [&'static str] = &[
    "en",  // English
    "es",  // Spanish (Español)
];


#[derive(Clone, Deserialize, Serialize)]
pub struct PreConfig {
    pub server: ConfigServer,
    pub database: ConfigDatabase,
    pub modules: ConfigModules,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigServer {
    pub server_os: String,
    pub mode: String,
    pub domain: String,
    pub new_domain: String,
    pub user_email: String,
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
pub struct ConfigModules {
    pub blog: ConfigModulesBlog,
    pub gallery: ConfigModulesGallery,
    pub faq: ConfigModulesFAQ,
    pub payments: ConfigModulesPayments,
    pub subscriptions: ConfigModulesSubscriptions,
    pub shop: ConfigModulesShop,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigModulesBlog {
    pub enabled: String,
    pub web_enabled: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigModulesGallery {
    pub enabled: String,
    pub web_enabled: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigModulesFAQ {
    pub enabled: String,
    pub web_enabled: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigModulesPayments {
    pub enabled: String,
    pub web_enabled: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigModulesSubscriptions {
    pub enabled: String,
    pub web_enabled: String,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ConfigModulesShop {
    pub enabled: String,
    pub web_enabled: String,
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
    pub modules: ConfigModules,
    pub dbauth: String,
}


pub fn config() -> Config {
    let toml_file = &fs::read_to_string("etc/Tukosmo.toml")
        .expect("Something went wrong reading etc/Tukosmo.toml!");
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
        modules: config.modules,
        dbauth: db_auth_string.to_string(),
    }
}

