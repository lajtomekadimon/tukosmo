use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::admin::scope_json;


pub fn route() -> Resource {
    // Files selector: /{lang}/admin/json/files_selector
    resource("/files_selector").route(
        get().to(scope_json::files_selector_get::files_selector_get)
    )
}

