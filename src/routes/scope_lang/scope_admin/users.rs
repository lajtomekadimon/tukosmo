use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::admin;
use crate::routes::scope_lang::scope_admin::scope_users;


pub fn route() -> Resource {
    // Users: /{lang}/admin/users
    resource("/users").route(
        get().to(admin::users_get::users_get)
    )
}


pub fn route_root() -> Resource {
    // Users: /{lang}/admin/users/
    resource("/").route(
        get().to(admin::users_get::users_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/users")
        // Users: /{lang}/admin/users/
        .service(route_root())

        // New user: /{lang}/admin/users/new
        .service(scope_users::new::route())

        // Edit user: /{lang}/admin/users/edit
        .service(scope_users::edit::route())

        // Delete user: /{lang}/admin/users/delete
        .service(scope_users::delete::route())
}
