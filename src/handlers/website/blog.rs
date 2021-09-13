use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::i18n::t::t;
use crate::templates::website::blog::Blog;
use crate::i18n::current_language::current_language;
use crate::database::aww_blog::aww_blog;


pub async fn blog(
    req: HttpRequest,
) -> impl Responder {
    if let Some(lang) = current_language(req) {

        let html = Blog {
            title: &format!(
                "{a} - {b}",
                a = &t("Blog", &lang.code),
                b = "MyExample"
            ),
            lang: &lang,
            posts: &aww_blog(lang.id),
        };

        HttpResponse::Ok().body(html.to_string())

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }
}

