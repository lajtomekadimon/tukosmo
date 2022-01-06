use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_users;


pub fn route() -> Resource {
    // Delete user: /{lang}/admin/users/delete
    resource("/delete").route(
        get().to(scope_users::delete_get::delete_get)
    ).route(
        post().to(scope_users::delete_post::delete_post)
    )
}

