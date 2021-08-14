use actix_web::{web, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;


#[derive(Deserialize)]
pub struct InfoData {
    lang: String,
}

pub async fn logout(
    id: Identity,
    web::Query(info): web::Query<InfoData>,
) -> impl Responder {
    let lang_code = (info.lang).clone();

    // Delete auth cookie
    id.forget();

    // TODO: Delete session from database!

    // Redirect to login page
    HttpResponse::Found()
        .header(
            "Location",
            "/{lang}/admin/login".replace("{lang}", &lang_code)
        )
        .finish()
}
