use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Logout: /{lang}/admin/logout
    resource("/logout").route(
        get().to(admin::logout_get::logout_get)
    )
}

