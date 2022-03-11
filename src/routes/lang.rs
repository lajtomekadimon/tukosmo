use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::website;
use crate::routes::scope_lang;


pub fn route() -> Resource {
    // Homepage: /{lang}
    resource("/{lang}").route(
        get().to(website::home_get::home_get)
    )
}


pub fn route_root() -> Resource {
    // Homepage: /{lang}/
    resource("/").route(
        get().to(website::home_get::home_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/{lang}")
        // Homepage: /{lang}/
        .service(route_root())

        // Blog: /{lang}/blog
        .service(scope_lang::blog::route())
        .service(scope_lang::blog::subroutes())

        // RSS: /{lang}/rss
        .service(scope_lang::rss::subroutes())

        // Page: /{lang}/page/{permalink}
        .service(scope_lang::page::route())

        // Error: /{lang}/error
        .service(scope_lang::error::route())

        // Admin Panel dashboard: /{lang}/admin
        .service(scope_lang::admin::route())
        .service(scope_lang::admin::subroutes())
}
