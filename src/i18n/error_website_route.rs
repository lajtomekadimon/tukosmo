use actix_web::HttpResponse;
use postgres::Error;

use crate::i18n::t_error::t_error;
use crate::handlers::website::error_get::rw_error_w_code;


pub fn error_website_route(
    pg_error: &Error,
    lang_code: &str,
) -> HttpResponse {
 
    let e = t_error(pg_error, lang_code);

    HttpResponse::Found()
        .header(
            "Location",
            rw_error_w_code(
                lang_code,
                &e.code,
            ),
        )
        .finish()

}
