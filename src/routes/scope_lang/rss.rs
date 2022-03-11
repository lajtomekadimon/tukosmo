use actix_web::Scope;
use actix_web::web::scope;

use crate::routes::scope_lang::scope_rss;


pub fn subroutes() -> Scope {
    scope("/rss")
        // Blog RSS: /{lang}/rss/blog
        .service(scope_rss::blog::route())
}
