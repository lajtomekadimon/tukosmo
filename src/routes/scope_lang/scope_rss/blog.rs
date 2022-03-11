use actix_web::Resource;
use actix_web::web::{resource, get};

use crate::handlers::website::scope_rss;


pub fn route() -> Resource {
    // Blog RSS: /{lang}/rss/blog
    resource("/blog").route(
        get().to(scope_rss::blog_get::blog_get)
    )
}
