use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::handlers::website::home_get::rw_home;
use crate::i18n::http_accept_language::http_accept_language;


pub async fn root_get(
    req: HttpRequest,
) -> impl Responder {
    let lang_code = http_accept_language(req);

    HttpResponse::Found().header(
        "Location",
        rw_home(&lang_code),
    ).finish()
}

