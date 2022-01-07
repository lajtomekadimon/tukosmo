use actix_web::Resource;
use actix_web::web::{resource, post};

use crate::handlers::admin::scope_json;


pub fn route() -> Resource {
    // Edit file: /{lang}/admin/json/edit_file
    resource("/edit_file").route(
        post().to(scope_json::edit_file_post::edit_file_post)
    )
}

