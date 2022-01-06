use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Favicon: /{lang}/admin/favicon
    resource("/favicon").route(
        get().to(admin::favicon_get::favicon_get)
    ).route(
        post().to(admin::favicon_post::favicon_post)
    )
}

