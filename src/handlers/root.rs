use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::i18n::http_accept_language::http_accept_language;
use crate::handlers::website::home::rw_home;


pub async fn root(
    req: HttpRequest,
) -> impl Responder {
    let lang_code = http_accept_language(req);

    HttpResponse::Found().header(
        "Location",
        rw_home(&lang_code),
    ).finish()
}

