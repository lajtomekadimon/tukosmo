use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::i18n::t::t;
use crate::templates::website::blog::Blog;


// TODO
#[get("/blog")]
async fn handler_blog(
    req: HttpRequest,
) -> impl Responder {
    let lang_code: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Blog {
        title: &t("Blog", &lang_code),
        lang_code: &lang_code,
    };

    HttpResponse::Ok().body(html.to_string())
}

