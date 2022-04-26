use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Tukosmo: /{lang}/admin/tukosmo
    resource("/tukosmo").route(
        get().to(admin::tukosmo_get::tukosmo_get)
    ).route(
        post().to(admin::tukosmo_post::tukosmo_post)
    )
}

