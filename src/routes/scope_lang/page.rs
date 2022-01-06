use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::website;


pub fn route() -> Resource {
    // Page: /{lang}/page/{permalink}
    resource("/page/{permalink}").route(
        get().to(website::page_get::page_get)
    )
}
