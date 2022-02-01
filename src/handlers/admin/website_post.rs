use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;
use std::sync::mpsc;

use crate::config::{
    global::Config,
    change_lang::change_lang,
};
use crate::handlers::admin::{
    user_request::user_request,
    website_get::{
        AgiWebsite,
        AgoWebsite,
    },
    error_get::ra_error_w_code,
    website_get::ra_website_success,
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
use crate::templates::admin::website::Website;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub website_title: String,
    pub website_subtitle: String,
    pub copyright_owner: String,
    pub default_lang: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiWebsite {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub website_title: String,
    pub website_subtitle: String,
    pub copyright_owner: String,
    pub default_lang: String,
}

impl QueryFunction for ApiWebsite {
    fn query(&self) -> &str {
        "SELECT aha_p_website($1)"
    }
}


pub async fn website_post(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
    restarter: web::Data<mpsc::Sender<()>>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let website_title = (form.website_title).clone();
                let website_subtitle = (form.website_subtitle).clone();
                let copyright_owner = (form.copyright_owner).clone();
                let default_lang = (form.default_lang).clone();

                match query_db(
                    &config,
                    ApiWebsite {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        website_title: website_title,
                        website_subtitle: website_subtitle,
                        copyright_owner: copyright_owner,
                        default_lang: default_lang.clone(),
                    },
                ) {

                    Ok(_row) => {

                        if &config.server.default_lang != &default_lang {
                            change_lang(
                                &config,
                                &default_lang,
                            );
                            // TODO: Handle errors

                            // Restart server
                            restarter.send(()).unwrap();
                        }

                        HttpResponse::Found()
                            .header(
                                "Location",
                                ra_website_success(&user_req.lang_code),
                            )
                            .finish()

                    },

                    Err(e) => match query_db(
                        &config,
                        AgiWebsite {
                            req: user_req.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: AgoWebsite = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = Website {
                                domain: &config.server.domain,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.website,
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                success: &false,
                                error: &Some(
                                    t_error(&e, &q.data.lang.code),
                                ),
                                form: &Some(form),
                                default_lang: &config.server.default_lang,
                            };

                            HttpResponse::Ok()
                                .content_type("text/html; charset=UTF-8")
                                .body(html.to_string())

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
