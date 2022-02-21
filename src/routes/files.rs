use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::files_get::files_get;


pub fn route() -> Resource {
    resource("/files/{codename}/{filename:.*}").route(
        get().to(files_get)
    )
}
