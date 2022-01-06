use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::website;


pub fn route() -> Resource {
    // Error: /{lang}/error
    resource("/error").route(
        get().to(website::error_get::error_get)
    )
}
