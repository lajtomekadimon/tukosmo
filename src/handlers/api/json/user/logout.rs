use actix_web::{web, Responder};
use serde_json::json;


pub async fn logout() -> impl Responder {

    web::Json(
        json!("Here I should delete the session! :D")
    )
}
