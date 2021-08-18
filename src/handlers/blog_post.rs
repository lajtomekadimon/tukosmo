use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::blog_post::BlogPost;
use crate::i18n::current_language::current_language;


pub async fn handler_blog_post(
    req: HttpRequest,
) -> impl Responder {
    if let Some(lang_code) = current_language(req) {

        let html = BlogPost {
            title: &format!(
                "{a} - {b}",
                a = "[post title]",
                b = "MyExample"
            ),
            lang_code: &lang_code,
        };

        HttpResponse::Ok().body(html.to_string())

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }
}

