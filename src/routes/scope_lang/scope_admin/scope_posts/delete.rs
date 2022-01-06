use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_posts;


pub fn route() -> Resource {
    // Delete post: /{lang}/admin/posts/delete
    resource("/delete").route(
        get().to(scope_posts::delete_get::delete_get)
    ).route(
        post().to(scope_posts::delete_post::delete_post)
    )
}

