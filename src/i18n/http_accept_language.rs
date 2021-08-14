use actix_web::HttpRequest;


pub fn http_accept_language(
    req: HttpRequest,
) -> String {
    let lang_code;

    if let Some(the_language) = req.headers().get("Accept-Language") {
        if let Ok(the_language_str) = the_language.to_str() {
            lang_code = the_language_str[..2].to_string();
        } else {
            lang_code = "en".to_string();
        }
    } else {
        lang_code = "en".to_string();
    }

    lang_code
}

