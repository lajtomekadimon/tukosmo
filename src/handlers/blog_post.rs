use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::blog_post::BlogPost;


pub async fn handler_blog_post(
    req: HttpRequest,
) -> impl Responder {
    let lang_code: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = BlogPost {
        title: &format!(
            "{a} - {b}",
            a = "[post title]",
            b = "MyExample"
        ),
        lang_code: &lang_code,
    };

    HttpResponse::Ok().body(html.to_string())
}

