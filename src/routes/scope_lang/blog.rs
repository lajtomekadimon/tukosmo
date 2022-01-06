use actix_web::{Resource, Scope};
use actix_web::web::{resource, get, scope};

use crate::handlers::website;
use crate::routes::scope_lang::scope_blog;


pub fn route() -> Resource {
    // Blog: /{lang}/blog
    resource("/blog").route(
        get().to(website::blog_get::blog_get)
    )
}


pub fn route_root() -> Resource {
    // Blog: /{lang}/blog/
    resource("/").route(
        get().to(website::blog_get::blog_get)
    )
}


pub fn subroutes() -> Scope {
    scope("/blog")
        // Blog: /{lang}/blog/
        .service(route_root())

        // Blog post: /{lang}/blog/{permalink}
        .service(scope_blog::blog_post::route())
}
