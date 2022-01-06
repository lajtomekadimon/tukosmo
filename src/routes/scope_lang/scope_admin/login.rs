use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Login: /{lang}/admin/login
    resource("/login").route(
        get().to(admin::login_get::login_get)
    ).route(
        post().to(admin::login_post::login_post)
    )
}

