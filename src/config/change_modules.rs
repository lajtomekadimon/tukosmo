use toml;
use std::fs;

use crate::config::global::{
    Config,
    PreConfig,
    ConfigModules,
    ConfigModulesBlog,
    ConfigModulesGallery,
    ConfigModulesFAQ,
    ConfigModulesPayments,
    ConfigModulesSubscriptions,
    ConfigModulesShop,
};


pub fn change_modules(
    config: &Config,
    module_blog: bool,
    module_gallery: bool,
    module_faq: bool,
    module_payments: bool,
    module_subscriptions: bool,
    module_shop: bool,
) {
    let new_toml_file = toml::to_string(
        &PreConfig {
            server: (&config.server).clone(),
            database: (&config.database).clone(),
            modules: ConfigModules {
                // Blog
                blog: if module_blog { ConfigModulesBlog {
                    enabled: "yes".to_string(),
                    web_enabled: "yes".to_string(),
                }} else { ConfigModulesBlog {
                    enabled: "no".to_string(),
                    web_enabled: "no".to_string(),
                }},

                // Gallery
                gallery: if module_gallery { ConfigModulesGallery {
                    enabled: "yes".to_string(),
                    web_enabled: "yes".to_string(),
                }} else { ConfigModulesGallery {
                    enabled: "no".to_string(),
                    web_enabled: "no".to_string(),
                }},

                // FAQ
                faq: if module_faq { ConfigModulesFAQ {
                    enabled: "yes".to_string(),
                    web_enabled: "yes".to_string(),
                }} else { ConfigModulesFAQ {
                    enabled: "no".to_string(),
                    web_enabled: "no".to_string(),
                }},

                // Payments
                payments: if module_payments { ConfigModulesPayments {
                    enabled: "yes".to_string(),
                    web_enabled: "yes".to_string(),
                }} else { ConfigModulesPayments {
                    enabled: "no".to_string(),
                    web_enabled: "no".to_string(),
                }},

                // Subscriptions
                subscriptions: if module_subscriptions {
                    ConfigModulesSubscriptions {
                        enabled: "yes".to_string(),
                        web_enabled: "yes".to_string(),
                    }
                } else {
                    ConfigModulesSubscriptions {
                        enabled: "no".to_string(),
                        web_enabled: "no".to_string(),
                    }
                },

                // Shop
                shop: if module_shop { ConfigModulesShop {
                    enabled: "yes".to_string(),
                    web_enabled: "yes".to_string(),
                }} else { ConfigModulesShop {
                    enabled: "no".to_string(),
                    web_enabled: "no".to_string(),
                }},
            },
        }
    ).unwrap();
    fs::write("etc/Tukosmo.toml", new_toml_file).unwrap();
}
