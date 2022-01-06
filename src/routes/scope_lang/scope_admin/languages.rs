use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::admin;
use crate::routes::scope_lang::scope_admin::scope_languages;


pub fn route() -> Resource {
    // Languages: /{lang}/admin/languages
    resource("/languages").route(
        get().to(admin::languages_get::languages_get)
    )
}


pub fn route_root() -> Resource {
    // Languages: /{lang}/admin/languages/
    resource("/").route(
        get().to(admin::languages_get::languages_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/languages")
        // Users: /{lang}/admin/languages/
        .service(route_root())

        // New language: /{lang}/admin/languages/new
        .service(scope_languages::new::route())

        // Edit language: /{lang}/admin/languages/edit
        .service(scope_languages::edit::route())

        // Delete language: /{lang}/admin/languages/delete
        .service(scope_languages::delete::route())
}
