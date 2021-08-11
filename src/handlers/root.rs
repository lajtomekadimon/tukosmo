use actix_web::{get, HttpResponse, Responder};


// TODO
#[get("/")]
async fn root() -> impl Responder {
    // TODO: Redirect to /en, /es, etc. using HTTP header's language
    HttpResponse::Found().header("Location", "/en/").finish()
}

