use actix_web::{post, HttpResponse, Responder};


// TODO
#[post("/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("Sign in!")
}
