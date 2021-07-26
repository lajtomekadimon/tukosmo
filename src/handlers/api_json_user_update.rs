use actix_web::{post, HttpResponse, Responder};


// TODO
#[post("/update")]
async fn handler_api_json_user_update() -> impl Responder {
    HttpResponse::Ok().body("Sign in!")
}
