use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Pages: /{lang}/admin/pages
    resource("/pages").route(
        get().to(admin::pages_get::pages_get)
    )
}

