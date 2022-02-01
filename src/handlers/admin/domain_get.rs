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
use crate::templates::admin::domain::Domain;


#[derive(Deserialize)]
pub struct GetParamData {
    nochange: Option<String>,
}


pub fn ra_domain(
    lang_code: &str,
) -> String {
    "/{lang}/admin/domain".replace("{lang}", lang_code)
}

pub fn ra_domain_nochange(
    lang_code: &str,
) -> String {
    "/{lang}/admin/domain?nochange=true"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiDomain {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiDomain {
    fn query(&self) -> &str {
        "SELECT aha_g_domain($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoDomain {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn domain_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiDomain {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: AgoDomain = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Domain {
                    domain: &config.server.domain,
                    title: &format!(
                        "{a} - {b}",
                        a = t.domain_k_web,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    nochange: match param.nochange {
                        Some(_) => &true,
                        None => &false,
                    },
                    error: &None,
                    form: &None,
                    user_email: &config.server.user_email,
                    is_development: &(&config.server.mode == "development"),
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

