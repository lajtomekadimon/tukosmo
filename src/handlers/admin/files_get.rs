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
use crate::templates::admin::files::Files;


#[derive(Deserialize)]
pub struct GetParamData {
    rpp: Option<i64>,
    p: Option<i64>,
    success: Option<String>,
}


pub fn ra_files(
    lang_code: &str,
) -> String {
    "/{lang}/admin/files".replace("{lang}", lang_code)
}

pub fn ra_files_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/files?success=true"
        .replace("{lang}", lang_code)
}

pub fn ra_files_wu_rpp_p(
    lang_code: &str,
) -> String {
    "/{lang}/admin/files?rpp={rpp}&p={p}"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiFiles {
    pub req: types::AdminRequest,
    pub results_per_page: i64,
    pub page: i64,
}

impl QueryFunction for AgiFiles {
    fn query(&self) -> &str {
        "SELECT aha_g_files($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoFiles {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub files: Vec<types::FileDB>,
    pub results_per_page: i64,
    pub page: i64,
    pub total_results: i64,
    pub total_pages: i64,
}


pub async fn files_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiFiles {
                req: user_req.clone(),
                results_per_page: results_per_page,
                page: current_page,
            },
        ).await {

            Ok(row) => {

                let q: AgoFiles = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Files {
                    domain: &config.server.domain,
                    codename: &codename,
                    title: &format!(
                        "{a} - {b}",
                        a = t.files,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: match param.success {
                        Some(_) => &true,
                        None => &false,
                    },
                };

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            },

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

