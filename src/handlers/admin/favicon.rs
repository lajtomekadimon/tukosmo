use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::favicon::Favicon;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct FaviconARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for FaviconARequest {
    fn query(&self) -> &str {
        "SELECT awa_favicon($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct FaviconAResponse {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn favicon(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            FaviconARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: FaviconAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Favicon {
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

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}
