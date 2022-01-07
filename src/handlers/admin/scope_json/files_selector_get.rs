use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use serde_json::json;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::t::t;
use crate::templates::admin::scope_json::files_selector::FilesSelector;


#[derive(Deserialize)]
pub struct GetParamData {
    rpp: Option<i64>,
    p: Option<i64>,
}


pub fn ra_json_files_selector_wu_rpp_p(
    lang_code: &str,
) -> String {
    "/{lang}/admin/json/files_selector?rpp={rpp}&p={p}"
        .replace("{lang}", lang_code)
}

pub fn ra_json_files_selector_w_rpp_p(
    lang_code: &str,
    rpp: &i64,
    p: &i64,
) -> String {
    "/{lang}/admin/json/files_selector?rpp={rpp}&p={p}"
        .replace("{lang}", lang_code)
        .replace("{rpp}", &rpp.to_string())
        .replace("{p}", &p.to_string())
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiJsonFilesSelector {
    pub req: types::AdminRequest,
    pub results_per_page: i64,
    pub page: i64,
}

impl QueryFunction for AgiJsonFilesSelector {
    fn query(&self) -> &str {
        "SELECT aha_g_json_files_selector($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoJsonFilesSelector {
    pub data: types::AdminDataDB,
    pub files: Vec<types::FileDB>,
    pub results_per_page: i64,
    pub page: i64,
    pub total_results: i64,
    pub total_pages: i64,
    pub csrf_token: String,
}


pub async fn files_selector_get(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            AgiJsonFilesSelector {
                req: user_req.clone(),
                results_per_page: results_per_page,
                page: current_page,
            },
        ) {

            Ok(row) => {

                let q: AgoJsonFilesSelector = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = FilesSelector {
                    q: &q,
                    t: t,
                };

                let body = json!({
                    "success": true,
                    "html": html.to_string(),
                });
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body.to_string())

            },

            Err(_e) => {
                let body = json!({
                    "success": false,
                    // TODO: Show error
                });
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body.to_string())
            },

        },

        Err(_redirect_url) => {
            let body = json!({
                "success": false,
                // TODO: Show error
            });
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body.to_string())
        },

    }

}

