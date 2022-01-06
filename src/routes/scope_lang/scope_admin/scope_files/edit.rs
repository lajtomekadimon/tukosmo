use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin::scope_files;


pub fn route() -> Resource {
    // Edit file: /{lang}/admin/file/edit
    resource("/edit").route(
        get().to(scope_files::edit_get::edit_get)
    ).route(
        post().to(scope_files::edit_post::edit_post)
    )
}

