use actix_web::{post, HttpResponse, Responder};


// TODO
#[post("/signin")]
async fn handler_api_json_user_signin() -> impl Responder {
    HttpResponse::Ok().body("Sign in!")
}
