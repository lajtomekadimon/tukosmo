use actix_web::HttpRequest;

use crate::database::c_lang_by_code::c_lang_by_code;


pub fn current_language(
    req: HttpRequest,
) -> Option<String> {
    let lang_code: String = req.match_info()
        .get("lang").unwrap().parse().unwrap();

    // Check if lang_code exists in database
    if let Ok(lang_exists) = c_lang_by_code(lang_code.clone()) {
        if lang_exists {
            Some(lang_code)
        } else {
            None
        }
    } else {
        None
    }
}

