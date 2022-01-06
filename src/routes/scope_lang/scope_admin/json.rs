use actix_web::Scope;
use actix_web::web::scope;

use crate::routes::scope_lang::scope_admin::scope_json;


pub fn subroutes() -> Scope {
    // JSON API
    scope("/json")
        // Files selector: /{lang}/admin/json/files_selector
        .service(scope_json::files_selector::route())
}
