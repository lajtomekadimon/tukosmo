use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_posts;


pub fn route() -> Resource {
    // New post: /{lang}/admin/posts/new
    resource("/new").route(
        get().to(scope_posts::new_get::new_get)
    ).route(
        post().to(scope_posts::new_post::new_post)
    )
}

