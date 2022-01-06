use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Account: /{lang}/admin/account
    resource("/account").route(
        get().to(admin::account_get::account_get)
    ).route(
        post().to(admin::account_post::account_post)
    )
}

