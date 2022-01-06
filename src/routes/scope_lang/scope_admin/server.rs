use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Server: /{lang}/admin/server
    resource("/server").route(
        get().to(admin::server_get::server_get)
    )
}

