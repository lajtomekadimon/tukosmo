use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_languages;


pub fn route() -> Resource {
    // New language: /{lang}/admin/languages/new
    resource("/new").route(
        get().to(scope_languages::new_get::new_get)
    )
    .route(
        post().to(scope_languages::new_post::new_post)
    )
}

