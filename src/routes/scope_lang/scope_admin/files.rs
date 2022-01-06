use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::admin;
use crate::routes::scope_lang::scope_admin::scope_files;


pub fn route() -> Resource {
    // Files: /{lang}/admin/files
    resource("/files").route(
        get().to(admin::files_get::files_get)
    )
}


pub fn route_root() -> Resource {
    // Files: /{lang}/admin/files/
    resource("/").route(
        get().to(admin::files_get::files_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/files")
        // Files: /{lang}/admin/files/
        .service(route_root())

        // New file: /{lang}/admin/files/new
        .service(scope_files::new::route())

        // Edit file: /{lang}/admin/files/edit
        .service(scope_files::edit::route())

        // Delete file: /{lang}/admin/files/delete
        .service(scope_files::delete::route())
}
