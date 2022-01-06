use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::root_get::root_get;


pub fn route() -> Resource {
    resource("/").route(
        get().to(root_get)
    )
}
