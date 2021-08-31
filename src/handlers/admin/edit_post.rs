use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::edit_post::EditPost;
use crate::database::s_post_by_id_lang::s_post_by_id_lang;


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

        Ok((lang_code, _user_id)) => {

            if let Some(post) = s_post_by_id_lang(
                post_id,
                lang_code.clone(),
            ) {
                let html = EditPost {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit post: '{title}'",
                            &lang_code
                        ).replace("{title}", &post.title),
                        b = &t("Tukosmo Admin Panel", &lang_code)
                    ),
                    lang_code: &lang_code,
                    post: &post,
                };

                HttpResponse::Ok().body(html.to_string())
            } else {
                panic!("AHHHH");
            }

        }

        Err(r) => {r}

    }

}

