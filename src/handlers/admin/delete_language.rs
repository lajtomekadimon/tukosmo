use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::delete_language::DeleteLanguage;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteLanguageARequest {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for DeleteLanguageARequest {
    fn query(&self) -> &str {
        "SELECT awa_delete_language($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteLanguageAResponse {
    pub data: types::AdminDataDB,
    pub lang: types::LanguageDB,
}


pub async fn delete_language(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let lang_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            DeleteLanguageARequest {
                req: user_req,
                id: lang_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: DeleteLanguageAResponse = row.get(0);

                let html = DeleteLanguage {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Delete language: {name}",
                            &q.data.lang.code
                        ).replace("{name}", &q.lang.name),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
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

