use actix_web::{post, web, Responder};
use serde_json::json;


// TODO
#[post("/update")]
async fn update() -> impl Responder {

    web::Json(
        json!("Here I should update the user! :D")
    )
}
