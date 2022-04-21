use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::scope_languages::delete::Delete;


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


pub fn ra_languages_delete_w_id(
    lang_code: &str,
    id: &i64,
) -> String {
    "/{lang}/admin/languages/delete?id={id}"
        .replace("{lang}", lang_code)
        .replace("{id}", &id.to_string())
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiLanguagesDelete {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for AgiLanguagesDelete {
    fn query(&self) -> &str {
        "SELECT aha_g_languages_delete($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoLanguagesDelete {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub lang: types::LanguageDB,
}


pub async fn delete_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let lang_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiLanguagesDelete {
                req: user_req.clone(),
                id: lang_id.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoLanguagesDelete = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Delete {
                    domain: &config.server.domain,
                    codename: &codename,
                    config: &config,
                    title: &format!(
                        "{a} - {b}",
                        a = t.delete_language_w_name
                            .replace("{name}", &q.lang.name),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                };

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            }

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

