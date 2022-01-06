use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_users;


pub fn route() -> Resource {
    // Edit user: /{lang}/admin/users/edit
    resource("/edit").route(
        get().to(scope_users::edit_get::edit_get)
    ).route(
        post().to(scope_users::edit_post::edit_post)
    )
}

