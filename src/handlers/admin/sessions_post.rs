use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::sessions::{
    SessionsARequest,
    SessionsAResponse,
};
use crate::templates::admin::sessions::Sessions;


#[derive(Deserialize)]
pub struct FormData {
    user_agent: String,
    date: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SessionsPostARequest {
    pub req: types::AdminRequest,
    pub user_agent: String,
    pub date: String,
}

impl QueryFunction for SessionsPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_sessions_post($1)"
    }
}


pub async fn sessions_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {

            let user_agent_value = (form.user_agent).clone();
            let date_value = (form.date).clone();

            match query_db(
                SessionsPostARequest {
                    req: user_req.clone(),
                    user_agent: user_agent_value,
                    date: date_value,
                },
            ) {

                Ok(_row) => match query_db(
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
                            success: &true,
                            error: &None,
                        };

                        HttpResponse::Ok().body(html.to_string())

                    }

                    Err(e2) => error_admin_route(e2, &user_req.lang_code),

                },

                Err(e) => match query_db(
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
                            success: &true,
                            error: &Some(t_error(e, &q.data.lang.code)),
                        };

                        HttpResponse::Ok().body(html.to_string())

                    }

                    Err(e2) => error_admin_route(e2, &user_req.lang_code),

                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}

