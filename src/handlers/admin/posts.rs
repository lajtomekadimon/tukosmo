use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::posts::Posts;
use crate::database::awa_posts::awa_posts;


pub async fn posts(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok(data) => {

            let html = Posts {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Posts", &data.lang.code),
                    b = &t("Tukosmo Admin Panel", &data.lang.code)
                ),
                data: &data,
                posts: &awa_posts(data.lang.id),
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}

