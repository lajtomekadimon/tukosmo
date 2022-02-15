use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::staticf_get::staticf_get;


pub fn route() -> Resource {
    resource("/static/{codename}/{filename:.*}").route(
        get().to(staticf_get)
    )
}
