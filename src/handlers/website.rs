use actix_web::{get, HttpResponse, Responder};


// TODO
#[get("/")]
async fn handler_website() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

