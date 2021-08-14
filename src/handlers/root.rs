use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::i18n::http_accept_language::http_accept_language;


pub async fn root(
    req: HttpRequest,
) -> impl Responder {
    let lang_code = http_accept_language(req);

    HttpResponse::Found().header(
        "Location",
        "/{lang}/".replace("{lang}", &lang_code)
    ).finish()
}

