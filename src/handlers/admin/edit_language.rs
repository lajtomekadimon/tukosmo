use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::edit_language::EditLanguage;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


pub fn ra_edit_language_w_id(
    lang_code: &str,
    id: &i64,
) -> String {
    "/{lang}/admin/edit_language?id={id}"
        .replace("{lang}", lang_code)
        .replace("{id}", &id.to_string())
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguageARequest {
    pub req: types::AdminRequest,
    pub lang: i64,
}

impl QueryFunction for EditLanguageARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_language($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguageAResponse {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
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
            EditLanguageARequest {
                req: user_req.clone(),
                lang: param.id,
            },
        ) {

            Ok(row) => {

                let q: EditLanguageAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = EditLanguage {
                    title: &format!(
                        "{a} - {b}",
                        a = t.edit_language_w_name
                            .replace("{name}", &q.lang.name),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

