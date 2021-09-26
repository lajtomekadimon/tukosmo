use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::edit_language::EditLanguage;
use crate::database::types;
use crate::database::query_db::query_db;


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguageAR {
    pub req: types::AdminRequest,
    pub lang: i64,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguageAH {
    pub data: types::AdminDataDB,
    pub lang: types::LanguageDB,
    pub names: Vec<types::NameDB>,
}


pub async fn edit_language(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            EditLanguageAR {
                req: user_req,
                lang: param.id,
            },
        ) {

            Ok(row) => {

                let q: EditLanguageAH = row.get(0);

                let html = EditLanguage {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit language: {name}",
                            &q.data.lang.code
                        ).replace("{name}", &q.lang.name),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(_e) => HttpResponse::Found()
                .header("Location", "/")  // TODO
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}

