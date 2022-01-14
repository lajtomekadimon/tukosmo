use actix_web::HttpRequest;

use crate::config::global::Config;


pub fn http_accept_language(
    config: &Config,
    req: HttpRequest,
) -> String {
    if let Some(the_language) = req.headers().get("Accept-Language") {
        if let Ok(the_language_str) = the_language.to_str() {
            the_language_str[..2].to_string()
        } else {
            config.server.default_lang.clone()
        }
    } else {
        config.server.default_lang.clone()
    }
}

