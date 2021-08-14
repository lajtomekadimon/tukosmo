use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::page::Page;


pub async fn handler_page(
    req: HttpRequest,
) -> impl Responder {
    let lang_code: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Page {
        title: &format!(
            "{a} - {b}",
            a = "[page title]",
            b = "MyExample"
        ),
        lang_code: &lang_code,
    };

    HttpResponse::Ok().body(html.to_string())
}

