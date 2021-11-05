use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::website::Website;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsiteARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for WebsiteARequest {
    fn query(&self) -> &str {
        "SELECT awa_website($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsiteAResponse {
    pub data: types::AdminDataDB,
    pub csrf_token: String,
    pub website_title: String,
    pub website_subtitle: String,
}


pub async fn website(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            WebsiteARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: WebsiteAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Website {
                    title: &format!(
                        "{a} - {b}",
                        a = t.website,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: match param.success {
                        Some(_) => &true,
                        None => &false,
                    },
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

