use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::i18n::t::t;
use crate::templates::admin::languages::Languages;
use crate::database::data::DataDB;
use crate::database::s_languages::s_languages;


pub async fn languages(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok((lang, user)) => {

            let data = DataDB {
                user: user,
                lang: lang.clone(),
                languages: s_languages(lang.id),
            };

            let html = Languages {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Languages", &lang.code),
                    b = &t("Tukosmo Admin Panel", &lang.code)
                ),
                data: &data,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}

