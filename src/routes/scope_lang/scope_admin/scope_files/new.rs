use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_files;


pub fn route() -> Resource {
    // New file: /{lang}/admin/files/new
    resource("/new").route(
        get().to(scope_files::new_get::new_get)
    ).route(
        post().to(scope_files::new_post::new_post)
    )
}

