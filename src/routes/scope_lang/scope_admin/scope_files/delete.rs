use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_files;


pub fn route() -> Resource {
    // Delete file: /{lang}/admin/files/delete
    resource("/delete").route(
        get().to(scope_files::delete_get::delete_get)
    ).route(
        post().to(scope_files::delete_post::delete_post)
    )
}

