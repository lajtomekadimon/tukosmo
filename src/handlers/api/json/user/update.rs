use actix_web::{web, Responder};
use serde_json::json;


pub async fn update() -> impl Responder {

    web::Json(
        json!("Here I should update the user! :D")
    )
}
