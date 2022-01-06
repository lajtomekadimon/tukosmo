use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Error: /{lang}/admin/error
    resource("/error").route(
        get().to(admin::error_get::error_get)
    )
}

