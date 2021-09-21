use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::i18n::t::t;
use crate::templates::website::blog::Blog;
use crate::handlers::website::website_handler::website_handler;
use crate::database::aww_blog::aww_blog;


pub async fn blog(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match website_handler(req, id) {

        Ok(data) => {

            let html = Blog {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Blog", &data.lang.code),
                    b = "MyExample"
                ),
                data: &data,
                posts: &aww_blog(data.lang.id),
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}

