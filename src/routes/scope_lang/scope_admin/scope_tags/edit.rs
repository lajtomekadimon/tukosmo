use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_tags;


pub fn route() -> Resource {
    // Edit tag: /{lang}/admin/tags/edit
    resource("/edit").route(
        get().to(scope_tags::edit_get::edit_get)
    ).route(
        post().to(scope_tags::edit_post::edit_post)
    )
}

