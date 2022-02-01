use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;
use std::sync::mpsc;

use crate::config::{
    global::Config,
    change_domain::change_domain,
};
use crate::handlers::admin::{
    user_request::user_request,
    domain_get::{
        AgiDomain,
        AgoDomain,
    },
    error_get::ra_error_w_code,
    domain_get::ra_domain_nochange,
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
use crate::templates::admin::domain::Domain;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub domain: String,
    pub email: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiDomain {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub domain: String,
    pub email: String,
}

impl QueryFunction for ApiDomain {
    fn query(&self) -> &str {
        "SELECT aha_p_domain($1)"
    }
}


pub async fn domain_post(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
    restarter: web::Data<mpsc::Sender<()>>,
) -> impl Responder {

    // TODO: Block it if config != production

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let domain_value = (form.domain).clone();
                let email_value = (form.email).clone();

                match query_db(
                    &config,
                    ApiDomain {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        domain: domain_value.clone(),
                        email: email_value.clone(),
                    },
                ) {

                    Ok(_row) => {

                        if &config.server.domain != &domain_value {
                            let t = &t(&user_req.lang_code);

                            change_domain(
                                &config,
                                &domain_value,
                                &email_value,
                            );
                            // TODO: Handle errors

                            // Restart server
                            restarter.send(()).unwrap();

                            HttpResponse::Ok().body(
                                t.please_visit_new_domain_w_domain
                                    .replace("{domain}", &domain_value)
                            )
                        } else {
                            HttpResponse::Found()
                                .header(
                                    "Location",
                                    ra_domain_nochange(&user_req.lang_code),
                                )
                                .finish()
                        }

                    },

                    Err(e) => match query_db(
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
                                nochange: &false,
                                error: &Some(
                                    t_error(&e, &q.data.lang.code),
                                ),
                                form: &Some(form),
                                user_email: &config.server.user_email,
                                is_development:
                                    &(&config.server.mode == "development"),
                            };

                            HttpResponse::Ok().body(html.to_string())

                        }

                        Err(e2) => error_admin_route(&e2, &user_req.lang_code),

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
