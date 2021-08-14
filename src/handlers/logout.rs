use actix_web::{HttpResponse, Responder};
use actix_identity::Identity;


pub async fn logout(
    id: Identity,
) -> impl Responder {
    // Delete auth cookie
    id.forget();

    // TODO: Delete session from database!

    // Redirect to login page
    HttpResponse::Found()
        .header("Location", "/en/admin/login")  // TODO: Correct {lang}
        .finish()
}
