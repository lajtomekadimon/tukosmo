use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_languages;


pub fn route() -> Resource {
    // Delete language: /{lang}/admin/languages/delete
    resource("/delete").route(
        get().to(scope_languages::delete_get::delete_get)
    )
    .route(
        post().to(scope_languages::delete_post::delete_post)
    )
}

