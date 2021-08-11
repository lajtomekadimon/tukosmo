use actix_web::{get, HttpResponse, Responder};
use actix_identity::Identity;


#[get("/logout")]
async fn logout(
    id: Identity,
) -> impl Responder {
    // Delete auth cookie
    id.forget();

    // Redirect to login page
    HttpResponse::Found()
        .header("Location", "/en/admin/login")
        .finish()
}
