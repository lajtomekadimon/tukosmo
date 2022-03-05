use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_tags;


pub fn route() -> Resource {
    // Delete tag: /{lang}/admin/tags/delete
    resource("/delete").route(
        get().to(scope_tags::delete_get::delete_get)
    )
    .route(
        post().to(scope_tags::delete_post::delete_post)
    )
}

