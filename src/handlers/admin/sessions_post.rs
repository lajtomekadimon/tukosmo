use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::{
    user_request::user_request,
    sessions_get::{
        AgiSessions,
        AgoSessions,
    },
    error_get::ra_error_w_code,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID,
};
use crate::i18n::{
    t::t,
    t_error::t_error,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::sessions::Sessions;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub user_agent: String,
    pub date: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiSessions {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub user_agent: String,
    pub date: String,
}

impl QueryFunction for ApiSessions {
    fn query(&self) -> &str {
        "SELECT aha_p_sessions($1)"
    }
}


pub async fn sessions_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let user_agent_value = (form.user_agent).clone();
                let date_value = (form.date).clone();

                match query_db(
                    ApiSessions {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        user_agent: user_agent_value,
                        date: date_value,
                    },
                ) {

                    Ok(_row) => match query_db(
                        AgiSessions {
                            req: user_req.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: AgoSessions = row.get(0);
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
                        AgiSessions {
                            req: user_req.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: AgoSessions = row.get(0);
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

            Err(_) => HttpResponse::Found()
                .header(
                    "Location",
                    ra_error_w_code(
                        &user_req.lang_code,
                        CSRF_TOKEN_IS_NOT_A_VALID_UUID,
                    ),
                )
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}

