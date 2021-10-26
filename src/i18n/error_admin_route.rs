use actix_web::HttpResponse;
use postgres::Error;

use crate::i18n::t_error::t_error;


pub fn error_admin_route(
    pg_error: Error,
    lang_code: &str,
) -> HttpResponse {
 
    println!("{}", pg_error);  // debugging purposes

    let e = t_error(pg_error, lang_code);

    HttpResponse::Found()
        .header("Location", "/{lang}/admin/error?code={code}"
            .replace("{lang}", lang_code)
            .replace("{code}", &e.code)
        )
        .finish()

}
