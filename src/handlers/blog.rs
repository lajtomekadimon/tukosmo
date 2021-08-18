use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::i18n::t::t;
use crate::templates::website::blog::Blog;
use crate::i18n::current_language::current_language;


pub async fn handler_blog(
    req: HttpRequest,
) -> impl Responder {
    if let Some(lang_code) = current_language(req) {

        let html = Blog {
            title: &format!(
                "{a} - {b}",
                a = &t("Blog", &lang_code),
                b = "MyExample"
            ),
            lang_code: &lang_code,
        };

        HttpResponse::Ok().body(html.to_string())

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }
}

