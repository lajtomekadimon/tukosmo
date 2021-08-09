use actix_web::{post, web, Responder};
use serde_json::json;


// TODO
#[post("/logout")]
async fn logout() -> impl Responder {

    web::Json(
        json!("Here I should delete the session! :D")
    )
}
