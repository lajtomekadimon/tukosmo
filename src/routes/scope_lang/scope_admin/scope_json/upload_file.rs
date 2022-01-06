use actix_web::Resource;
use actix_web::web::{resource, post};

use crate::handlers::admin::scope_json;


pub fn route() -> Resource {
    // Files selector: /{lang}/admin/json/upload_file
    resource("/upload_file").route(
        post().to(scope_json::upload_file_post::upload_file_post)
    )
}

