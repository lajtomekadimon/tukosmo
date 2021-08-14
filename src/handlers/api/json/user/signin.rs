use actix_web::{web, Responder};
use serde_json::json;


pub async fn signin() -> impl Responder {

    web::Json(
        json!("Here I should do the sign in! :D")
    )
}
