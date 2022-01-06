use actix_web::Resource;
use actix_web::web::{resource, get, post};

use crate::handlers::admin;


pub fn route() -> Resource {
    // Sessions: /{lang}/admin/sessions
    resource("/sessions").route(
        get().to(admin::sessions_get::sessions_get)
    ).route(
        post().to(admin::sessions_post::sessions_post)
    )
}

