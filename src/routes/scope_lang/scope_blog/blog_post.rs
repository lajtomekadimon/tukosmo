use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::website::scope_blog;


pub fn route() -> Resource {
    // Blog post: /{lang}/blog/{permalink}
    resource("/{permalink}").route(
        get().to(scope_blog::post_get::post_get)
    )
}
