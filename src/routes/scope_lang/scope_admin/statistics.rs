use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Statistics: /{lang}/admin/statistics
    resource("/statistics").route(
        get().to(admin::statistics_get::statistics_get)
    )
}

