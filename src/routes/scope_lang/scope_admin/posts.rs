use actix_web::{Resource, Scope};
use actix_web::web::{resource, scope, get};

use crate::handlers::admin;
use crate::routes::scope_lang::scope_admin::scope_posts;


pub fn route() -> Resource {
    // Posts: /{lang}/admin/posts
    resource("/posts").route(
        get().to(admin::posts_get::posts_get)
    )
}


pub fn route_root() -> Resource {
    // Posts: /{lang}/admin/posts/
    resource("/").route(
        get().to(admin::posts_get::posts_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/posts")
        // Posts: /{lang}/admin/posts/
        .service(route_root())

        // New post: /{lang}/admin/posts/new
        .service(scope_posts::new::route())

        // Edit post: /{lang}/admin/posts/edit
        .service(scope_posts::edit::route())

        // Delete post: /{lang}/admin/posts/delete
        .service(scope_posts::delete::route())
}
