use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::edit_language::EditLanguage;
use crate::database::s_lang_code_by_id::s_lang_code_by_id;


#[derive(Deserialize)]
pub struct InfoData {
    id: i64,
}

pub async fn edit_language(
    req: HttpRequest,
    id: Identity,
    web::Query(info): web::Query<InfoData>,
) -> impl Responder {
    let lang_id = (info.id).clone();

    match admin_handler(req, id) {

        Ok((lang_code, user)) => {

            if let Some(lang_id_code) = s_lang_code_by_id(lang_id) {
                let html = EditLanguage {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit language: '{lang}'",
                            &lang_code
                        ).replace("{lang}", &lang_id_code),
                        b = &t("Tukosmo Admin Panel", &lang_code)
                    ),
                    lang_code: &lang_code,
                    lang_id: &lang_id,
                    lang_id_code: &lang_id_code,
                    user: &user,
                };

                HttpResponse::Ok().body(html.to_string())
            } else {
                panic!("AHHHH");
            }

        }

        Err(r) => {r}

    }

}

