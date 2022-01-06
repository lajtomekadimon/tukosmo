use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_languages;


pub fn route() -> Resource {
    // Edit language: /{lang}/admin/languages/edit
    resource("/edit").route(
        get().to(scope_languages::edit_get::edit_get)
    ).route(
        post().to(scope_languages::edit_post::edit_post)
    )
}

