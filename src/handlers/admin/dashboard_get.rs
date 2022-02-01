use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
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
use crate::templates::admin::dashboard::Dashboard;


pub fn ra_dashboard(
    lang_code: &str,
) -> String {
    "/{lang}/admin".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiDashboard {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiDashboard {
    fn query(&self) -> &str {
        "SELECT aha_g_dashboard($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoDashboard {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn dashboard_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiDashboard {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: AgoDashboard = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Dashboard {
                    domain: &config.server.domain,
                    title: t.tukosmo_admin_panel,
                    q: &q,
                    t: t,
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

