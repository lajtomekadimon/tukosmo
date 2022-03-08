use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::config::global::Config;
use crate::handlers::website::home_get::rw_home;
use crate::i18n::http_accept_language::http_accept_language;


pub async fn root_get(
    config: web::Data<Config>,
    req: HttpRequest,
) -> impl Responder {
    let lang_code = http_accept_language(&config, req);

    HttpResponse::Found().append_header((
        "Location",
        rw_home(&lang_code),
    )).finish()
}

