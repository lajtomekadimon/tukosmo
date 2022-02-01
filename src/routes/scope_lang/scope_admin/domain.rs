use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Domain: /{lang}/admin/domain
    resource("/domain").route(
        get().to(admin::domain_get::domain_get)
    ).route(
        post().to(admin::domain_post::domain_post)
    )
}

