use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::admin;
use crate::routes::scope_lang::scope_admin;


pub fn route() -> Resource {
    // Admin Panel dashboard: /{lang}/admin
    resource("/admin").route(
        get().to(admin::dashboard_get::dashboard_get)
    )
}


pub fn route_root() -> Resource {
    // Admin Panel dashboard: /{lang}/admin/
    resource("/").route(
        get().to(admin::dashboard_get::dashboard_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/admin")
        // Dashboard: /{lang}/admin/
        .service(route_root())

        // Error: /{lang}/admin/error
        .service(scope_admin::error::route())

        // Login: /{lang}/admin/login
        .service(scope_admin::login::route())

        // Logout: /{lang}/admin/logout
        .service(scope_admin::logout::route())

        // Account: /{lang}/admin/account
        .service(scope_admin::account::route())

        // Sessions: /{lang}/admin/sessions
        .service(scope_admin::sessions::route())

        //-- General --//

        // Statistics: /{lang}/admin/statistics
        .service(scope_admin::statistics::route())

        // Server: /{lang}/admin/server
        .service(scope_admin::server::route())

        //-- Data --//

        // Users: /{lang}/admin/users
        .service(scope_admin::users::route())
        .service(scope_admin::users::subroutes())

        // Languages: /{lang}/admin/languages
        .service(scope_admin::languages::route())
        .service(scope_admin::languages::subroutes())

        // Posts: /{lang}/admin/posts
        .service(scope_admin::posts::route())
        .service(scope_admin::posts::subroutes())

        // Pages: /{lang}/admin/pages
        .service(scope_admin::pages::route())

        // Files: /{lang}/admin/files
        .service(scope_admin::files::route())
        .service(scope_admin::files::subroutes())

        //-- Appearance --//

        // Favicon: /{lang}/admin/favicon
        .service(scope_admin::favicon::route())

        //-- Settings --//

        // Website: /{lang}/admin/website
        .service(scope_admin::website::route())

        // Tukosmo: /{lang}/admin/tukosmo
        .service(scope_admin::tukosmo::route())

        // JSON API: /{lang}/admin/json/*
        .service(scope_admin::json::subroutes())
}
