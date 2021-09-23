use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::edit_language::EditLanguage;
use crate::database::awa_edit_language::awa_edit_language;


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

        Ok(data) => {

            if let Some(lang_wnames) = awa_edit_language(
                lang_id.clone(),
                data.lang.id,
            ) {
                let html = EditLanguage {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit language: {name}",
                            &data.lang.code
                        ).replace("{name}", &lang_wnames.name),
                        b = &t("Tukosmo Admin Panel", &data.lang.code)
                    ),
                    data: &data,
                    lang_wnames: &lang_wnames,
                };

                HttpResponse::Ok().body(html.to_string())
            } else {
                panic!("AHHHH");  // TODO: Error 404
            }

        }

        Err(r) => {r}

    }

}

