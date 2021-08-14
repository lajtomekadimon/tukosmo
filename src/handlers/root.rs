use actix_web::{HttpResponse, Responder};


pub async fn root() -> impl Responder {
    HttpResponse::Found().header(
        "Location",
        "/{lang}/".replace("{lang}", "en")  // TODO: change "en" to HTTP's header language
    ).finish()
}

