use actix_web::HttpRequest;

use crate::database::s_current_language_by_code::s_current_language_by_code;
use crate::database::types::LanguageDB;


pub fn current_language(
    req: HttpRequest,
) -> Option<LanguageDB> {
    let lang_code: String = req.match_info()
        .get("lang").unwrap().parse().unwrap();

    s_current_language_by_code(lang_code)
}

