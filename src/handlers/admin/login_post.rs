use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::website::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::handlers::admin::login::{
    LoginARequest,
    LoginAResponse,
};
use crate::templates::admin::login::Login;


#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
    pub password: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginPostARequest {
    pub req: types::WebsiteRequest,
    pub email: String,
    pub password: String,
    pub user_agent: String,
}

impl QueryFunction for LoginPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_login_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginPostAResponse {
    pub data: types::WebsiteDataDB,
    pub session: Uuid,
}


pub async fn login_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    let user_req = user_request(req.clone(), id.clone());

    let email_value = (form.email).clone();
    let password_value = (form.password).clone();

    let user_agent_value = match req.headers().get("User-Agent") {
        Some(the_user_agent) => {
            match the_user_agent.to_str() {
                Ok(ua_value) => ua_value.to_string(),
                _ => "unknown".to_string(),
            }
        },
        None => "unknown".to_string(),
    };

    match query_db(
        LoginPostARequest {
            req: user_req.clone(),
            email: email_value,
            password: password_value, // TODO: This is not encrypted!!!
                                      /* Currently, password check is done
                                       * in the database. It may be more
                                       * secure to do this in the web
                                       * server, but the downside is that
                                       * instead of one query, we would
                                       * need to do two queries: one for
                                       * the password check, and another
                                       * one for the new session.
                                       */
            user_agent: user_agent_value,
        },
    ) {

        Ok(row) => {

            let q: LoginPostAResponse = row.get(0);
            //let t = &t(&q.data.lang.code);

            let encode_buffer_value = &mut Uuid::encode_buffer();

            let session_id_up = (q.session)
                .to_simple()
                .encode_upper(encode_buffer_value);

            id.remember(session_id_up.to_owned());

            HttpResponse::Found()
                .header(
                    "Location",
                    "/{lang}/admin/".replace("{lang}", &q.data.lang.code),
                )
                .finish()
        },

        Err(e) => match query_db(
            LoginARequest {
                req: user_req,
            },
        ) {

            Ok(row) => {

                let q: LoginAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                if let Some(_user) = q.data.userd {

                    let dashboard_route = "/{lang}/admin/"
                        .replace("{lang}", &q.data.lang.code);

                    HttpResponse::Found()
                        .header("Location", dashboard_route)
                        .finish()

                } else {

                    // Delete cookie
                    id.forget();

                    let html = Login {
                        title: &format!(
                            "{a} - {b}",
                            a = t.login_k_noun,
                            b = t.tukosmo_admin_panel,
                        ),
                        q: &q,
                        t: t,
                        error: &Some(t_error(e, &q.data.lang.code)),
                        form: &Some(form),
                    };

                    HttpResponse::Ok().body(html.to_string())

                }

            },

            Err(e2) => {
                println!("{}", e2);
                HttpResponse::Found()
                    .header("Location", "/")  // TODO
                    .finish()
            },

        },

    }

}
