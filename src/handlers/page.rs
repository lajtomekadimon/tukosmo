use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::templates::website::page::Page;


// TODO
#[get("/page")]
async fn handler_page(
    req: HttpRequest,
) -> impl Responder {
    let lang_value: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Page {
        title: "Tukosmo page",
        lang_code: &lang_value,
    };

    HttpResponse::Ok().body(html.to_string())
}

