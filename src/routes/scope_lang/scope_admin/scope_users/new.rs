use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_users;


pub fn route() -> Resource {
    // New user: /{lang}/admin/users/new
    resource("/new").route(
        get().to(scope_users::new_get::new_get)
    ).route(
        post().to(scope_users::new_post::new_post)
    )
}

