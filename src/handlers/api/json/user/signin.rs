use actix_web::{post, HttpResponse, Responder};


// TODO
#[post("/signin")]
async fn signin() -> impl Responder {
    HttpResponse::Ok().body("Sign in!")
}
