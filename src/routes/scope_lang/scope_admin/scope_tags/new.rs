use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_tags;


pub fn route() -> Resource {
    // New tag: /{lang}/admin/tags/new
    resource("/new").route(
        get().to(scope_tags::new_get::new_get)
    )
    .route(
        post().to(scope_tags::new_post::new_post)
    )
}

