use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::edit_post::EditPost;
use crate::database::awa_edit_post::awa_edit_post;
use crate::database::data::PostDB;


#[derive(Deserialize)]
pub struct InfoData {
    id: i64,
}

pub async fn edit_post(
    req: HttpRequest,
    id: Identity,
    web::Query(info): web::Query<InfoData>,
) -> impl Responder {
    let post_id = (info.id).clone();

    match admin_handler(req, id) {

        Ok(data) => {

            if let Some(post) = awa_edit_post(
                post_id,
                data.lang.id.clone(),
            ) {
                let html = EditPost {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit post: '{title}'",
                            &data.lang.code
                        ).replace("{title}", &post.title),
                        b = &t("Tukosmo Admin Panel", &data.lang.code)
                    ),
                    post: &post,
                    data: &data,
                };

                HttpResponse::Ok().body(html.to_string())
            } else {
                let html = EditPost {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit post: '{title}'",
                            &data.lang.code
                        ).replace("{title}", &post_id.to_string()),
                        b = &t("Tukosmo Admin Panel", &data.lang.code)
                    ),
                    post: &PostDB{
                        id: post_id,
                        trans_id: 0,
                        lang: 0,
                        title: "".to_string(),
                        description: "".to_string(),
                        body: "".to_string(),
                        permalink: "".to_string(),
                        author: 0,
                        author_name: "".to_string(),
                        translator: data.userd.id,
                        translator_name: "".to_string(),
                        date: "".to_string(),
                        date_trans: "".to_string(),
                        draft: false,
                        deleted: false,
                    },
                    data: &data,
                };

                HttpResponse::Ok().body(html.to_string())
            }

        }

        Err(r) => {r}

    }

}

