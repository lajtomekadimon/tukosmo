use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Forgotten password?: /{lang}/admin/forgotten_password
    resource("/forgotten_password").route(
        get().to(admin::forgotten_password_get::forgotten_password_get)
    ).route(
        post().to(admin::forgotten_password_post::forgotten_password_post)
    )
}

