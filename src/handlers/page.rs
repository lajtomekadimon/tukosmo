use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::page::Page;
use crate::i18n::current_language::current_language;


pub async fn handler_page(
    req: HttpRequest,
) -> impl Responder {
    if let Some(lang) = current_language(req) {

        let html = Page {
            title: &format!(
                "{a} - {b}",
                a = "[page title]",
                b = "MyExample"
            ),
            lang: &lang,
        };

        HttpResponse::Ok().body(html.to_string())

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }
}

