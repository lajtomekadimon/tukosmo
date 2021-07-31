use actix_web::{get, HttpResponse, Responder};

use crate::templates::website::Home;


// TODO
#[get("/")]
async fn handler_website() -> impl Responder {
    let html = Home {
        title: "Tukosmo website"
    };

    HttpResponse::Ok().body(html.to_string())
}

