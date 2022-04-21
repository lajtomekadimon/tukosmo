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
use crate::templates::admin::tags::Tags;


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


pub fn ra_tags(
    lang_code: &str,
) -> String {
    "/{lang}/admin/tags".replace("{lang}", lang_code)
}

pub fn ra_tags_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/tags?success=true"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiTags {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiTags {
    fn query(&self) -> &str {
        "SELECT aha_g_tags($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoTags {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub tags: Vec<types::TagDB>,
}


pub async fn tags_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiTags {
                req: user_req.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoTags = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Tags {
                    domain: &config.server.domain,
                    codename: &codename,
                    config: &config,
                    title: &format!(
                        "{a} - {b}",
                        a = t.tags,
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

            }

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

