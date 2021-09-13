use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::blog::Blog;
use crate::i18n::current_language::current_language;


pub async fn home(
    req: HttpRequest,
) -> impl Responder {
    if let Some(lang) = current_language(req) {

        let html = Blog {
            title: "MyExample - MySubtitle",
            lang: &lang,
        };

        HttpResponse::Ok().body(html.to_string())

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }
}

