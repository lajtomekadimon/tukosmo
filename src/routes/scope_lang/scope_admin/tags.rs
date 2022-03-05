use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::admin;
use crate::routes::scope_lang::scope_admin::scope_tags;


pub fn route() -> Resource {
    // Tags: /{lang}/admin/tags
    resource("/tags").route(
        get().to(admin::tags_get::tags_get)
    )
}


pub fn route_root() -> Resource {
    // Tags: /{lang}/admin/tags/
    resource("/").route(
        get().to(admin::tags_get::tags_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/tags")
        // Tags: /{lang}/admin/tags/
        .service(route_root())

        // New tag: /{lang}/admin/tags/new
        .service(scope_tags::new::route())

        // Edit tag: /{lang}/admin/tags/edit
        .service(scope_tags::edit::route())

        // Delete tag: /{lang}/admin/tags/delete
        .service(scope_tags::delete::route())
}
