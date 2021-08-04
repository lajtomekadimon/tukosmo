use actix_web::{get, HttpResponse, Responder};

use crate::templates::admin_layout::Home;


// TODO
#[get("/admin")]
async fn handler_admin() -> impl Responder {
    let html = Home {
        title: "Example domain"
    };

    HttpResponse::Ok().body(html.to_string())
}

