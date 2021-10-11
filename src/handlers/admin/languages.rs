use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::languages::Languages;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LanguagesARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for LanguagesARequest {
    fn query(&self) -> &str {
        "SELECT awa_languages($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LanguagesAResponse {
    pub data: types::AdminDataDB,
    pub some_lang_without_names: bool,
}


pub async fn languages(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            LanguagesARequest {
                req: user_req,
            },
        ) {

            Ok(row) => {

                let q: LanguagesAResponse = row.get(0);

                let html = Languages {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("Languages", &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    success: match param.success {
                        Some(_) => &true,
                        None => &false,
                    },
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => {
                println!("{}", e);
                HttpResponse::Found()
                    .header("Location", "/")  // TODO
                    .finish()
            },

        },

        Err(redirect_url) => redirect_url,

    }

}

