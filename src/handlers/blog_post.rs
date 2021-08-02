use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::templates::website::blog_post::BlogPost;


// TODO
#[get("/blog/{post}")]
async fn handler_blog_post(
    req: HttpRequest,
) -> impl Responder {
    let lang_value: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = BlogPost {
        title: "Tukosmo blog post",
        lang_code: &lang_value,
    };

    HttpResponse::Ok().body(html.to_string())
}

