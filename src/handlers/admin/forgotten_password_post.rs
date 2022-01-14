use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::{
    website::user_request::user_request,
    admin::forgotten_password_get::{
        AgiForgottenPassword,
        AgoForgottenPassword,
    },
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    t_error::t_error,
};
use crate::templates::admin::forgotten_password::ForgottenPassword;


#[derive(Deserialize)]
pub struct FormData {
    pub email: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiForgottenPassword {
    pub req: types::WebsiteRequest,
    pub email: String,
}

impl QueryFunction for ApiForgottenPassword {
    fn query(&self) -> &str {
        "SELECT aha_p_forgotten_password($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApoForgottenPassword {
    pub data: types::WebsiteDataDB,
    pub code: String,
}


pub async fn forgotten_password_post(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    let user_req = user_request(req.clone(), id.clone());

    let email_value = (form.email).clone();

    // TODO: Comprobar que el usuario no está conectado
    match query_db(
        &config,
        ApiForgottenPassword {
            req: user_req.clone(),
            email: email_value,
        },
    ) {

        Ok(_row) => match query_db(
            &config,
            AgiForgottenPassword {
                req: user_req,
            },
        ) {

            Ok(row2) => {

                //let q: ApoForgottenPassword = row.get(0);

                // TODO: Send email to email_value with the link (using q.code)

                let q2: AgoForgottenPassword = row2.get(0);
                let t = &t(&q2.data.lang.code);

                let html = ForgottenPassword {
                    domain: &config.server.domain,
                    title: &format!(
                        "{a} - {b}",
                        a = t.forgotten_password,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q2,
                    t: t,
                    error: &None,
                    form: &Some(form),
                    success: &true,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e2) => {
                println!("{}", e2);
                HttpResponse::Found()
                    .header("Location", "/")  // TODO
                    .finish()
            },

        },

        Err(e) => match query_db(
            &config,
            AgiForgottenPassword {
                req: user_req,
            },
        ) {

            Ok(row) => {

                let q: AgoForgottenPassword = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = ForgottenPassword {
                    domain: &config.server.domain,
                    title: &format!(
                        "{a} - {b}",
                        a = t.forgotten_password,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &Some(t_error(&e, &q.data.lang.code)),
                    form: &Some(form),
                    success: &false,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(_e2) => {
                HttpResponse::Found()
                    .header("Location", "/")  // TODO
                    .finish()
            },

        },

    }

}
