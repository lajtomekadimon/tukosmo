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
use crate::templates::admin::favicon::Favicon;


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


pub fn ra_favicon(
    lang_code: &str,
) -> String {
    "/{lang}/admin/favicon".replace("{lang}", lang_code)
}

pub fn ra_favicon_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/favicon?success=true"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiFavicon {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiFavicon {
    fn query(&self) -> &str {
        "SELECT aha_g_favicon($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoFavicon {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn favicon_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiFavicon {
                req: user_req.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoFavicon = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Favicon {
                    domain: &config.server.domain,
                    codename: &codename,
                    title: &format!(
                        "{a} - {b}",
                        a = t.favicon,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
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

