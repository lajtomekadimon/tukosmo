use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Website: /{lang}/admin/website
    resource("/website").route(
        get().to(admin::website_get::website_get)
    ).route(
        post().to(admin::website_post::website_post)
    )
}

