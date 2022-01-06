use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_posts;


pub fn route() -> Resource {
    // Edit post: /{lang}/admin/posts/edit
    resource("/edit").route(
        get().to(scope_posts::edit_get::edit_get)
    ).route(
        post().to(scope_posts::edit_post::edit_post)
    )
}

