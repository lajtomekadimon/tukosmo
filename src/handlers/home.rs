use actix_web::{get, HttpRequest, HttpResponse, Responder};

use crate::templates::home::Home;


// TODO
#[get("/")]
async fn handler_home(
    req: HttpRequest,
) -> impl Responder {
    let lang_value: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let html = Home {
        title: "Tukosmo website",
        lang_code: &lang_value
    };

    HttpResponse::Ok().body(html.to_string())
}

