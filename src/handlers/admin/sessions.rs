use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::sessions::Sessions;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SessionsARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for SessionsARequest {
    fn query(&self) -> &str {
        "SELECT awa_sessions($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SessionsAResponse {
    pub data: types::AdminDataDB,
    pub sessions: Vec<types::SessionDB>,
}


pub async fn sessions(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            SessionsARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: SessionsAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Sessions {
                    title: &format!(
                        "{a} - {b}",
                        a = t.sessions,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: &false,
                    error: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

