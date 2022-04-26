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
use crate::templates::admin::languages::Languages;


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


pub fn ra_languages(
    lang_code: &str,
) -> String {
    "/{lang}/admin/languages".replace("{lang}", lang_code)
}

pub fn ra_languages_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/languages?success=true"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiLanguages {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiLanguages {
    fn query(&self) -> &str {
        "SELECT aha_g_languages($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoLanguages {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub some_lang_without_names: bool,
}


pub async fn languages_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiLanguages {
                req: user_req.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoLanguages = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Languages {
                    domain: &config.server.domain,
                    codename: &codename,
                    config: &config,
                    title: &format!(
                        "{a} - {b}",
                        a = t.languages,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: &param.success.is_some(),
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

