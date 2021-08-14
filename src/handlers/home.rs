use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::blog::Blog;


pub async fn handler_home(
    req: HttpRequest,
) -> impl Responder {
    let lang_code: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Blog {
        title: "MyExample",
        lang_code: &lang_code,
    };

    HttpResponse::Ok().body(html.to_string())
}

