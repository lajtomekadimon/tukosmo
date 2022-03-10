use actix_web::HttpResponse;
use tokio_postgres::Error;

use crate::i18n::t_error::t_error;
use crate::handlers::{
    admin::error_get::ra_error_w_code,
    website::error_get::rw_error_w_code,
};
use crate::database::error_codes::USER_IS_NOT_ADMIN;


pub fn error_admin_route(
    pg_error: &Error,
    lang_code: &str,
) -> HttpResponse {
 
    let e = t_error(pg_error, lang_code);

    HttpResponse::Found()
        .append_header((
            "Location",
            if &e.code == USER_IS_NOT_ADMIN {
                rw_error_w_code(
                    lang_code,
                    &e.code,
                )
            } else {
                ra_error_w_code(
                    lang_code,
                    &e.code,
                )
            },
        ))
        .finish()

}
