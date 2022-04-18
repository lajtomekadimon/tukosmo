use actix_web::HttpRequest;
use url::Url;

use crate::i18n::{
    get_browser_from_user_agent::get_browser_from_user_agent,
    get_os_from_user_agent::get_os_from_user_agent,
};
use crate::database::types::HTTPDataDB;


pub fn http_data_request(
    req: HttpRequest,
) -> HTTPDataDB {

    let ip_value = match req.peer_addr() {
        Some(socket_addr_val) => socket_addr_val.ip().to_string(),
        None => "unknown".to_string(),
    };

    let referrer_value = match req.headers().get("Referer") {
        Some(the_referrer) => {
            match the_referrer.to_str() {
                Ok(ref_value) => match Url::parse(ref_value) {
                    Ok(url) => match url.host_str() {
                        Some(host_value) => host_value.to_string(),
                        None => "unknown".to_string(),
                    },
                    Err(_) => "unknown".to_string(),
                },
                _ => "unknown".to_string(),
            }
        },
        None => "unknown".to_string(),
    };

    let user_agent_value = match req.headers().get("User-Agent") {
        Some(the_user_agent) => {
            match the_user_agent.to_str() {
                Ok(ua_value) => ua_value.to_string(),
                _ => "unknown".to_string(),
            }
        },
        None => "unknown".to_string(),
    };

    let browser_value = get_browser_from_user_agent(&user_agent_value);
    let platform_value = get_os_from_user_agent(&user_agent_value);

    HTTPDataDB {
        ip: ip_value,
        referrer: referrer_value,
        browser: browser_value,
        platform: platform_value,
    }

}

