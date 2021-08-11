use actix_web::{post, web, Responder};
use serde_json::json;


// TODO
#[post("/signin")]
async fn signin() -> impl Responder {

    web::Json(
        json!("Here I should do the sign in! :D")
    )
}
