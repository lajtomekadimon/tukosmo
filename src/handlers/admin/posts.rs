use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::posts::Posts;
use crate::database::awa_posts::awa_posts;


#[derive(Deserialize)]
pub struct InfoData {
    p: Option<i64>,
}

pub async fn posts(
    req: HttpRequest,
    id: Identity,
    web::Query(info): web::Query<InfoData>,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok(data) => {

            let results_number = 10;
            let current_page = (info.p).clone().unwrap_or(1);

            let html = Posts {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Posts", &data.lang.code),
                    b = &t("Tukosmo Admin Panel", &data.lang.code)
                ),
                data: &data,
                posts: &awa_posts(
                    data.lang.id,
                    results_number,
                    current_page,
                ),
                current_page: &current_page,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}

