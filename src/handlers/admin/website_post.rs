use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::database::error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::website::{
    WebsiteARequest,
    WebsiteAResponse,
};
use crate::templates::admin::website::Website;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub website_title: String,
    pub website_subtitle: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsitePostARequest {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub website_title: String,
    pub website_subtitle: String,
}

impl QueryFunction for WebsitePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_website_post($1)"
    }
}


pub async fn website_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

            Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let website_title = (form.website_title).clone();
                let website_subtitle = (form.website_subtitle).clone();

                match query_db(
                    WebsitePostARequest {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        website_title: website_title,
                        website_subtitle: website_subtitle,
                    },
                ) {

                    Ok(_row) => {

                        let redirect_route =
                            "/{lang}/admin/website?success=yes"
                                .replace("{lang}", &user_req.lang_code);

                        HttpResponse::Found()
                            .header("Location", redirect_route)
                            .finish()

                    },

                    Err(e) => match query_db(
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
                                success: &false,
                                error: &Some(
                                    t_error(e, &q.data.lang.code),
                                ),
                                form: &Some(form),
                            };

                            HttpResponse::Ok().body(html.to_string())

                        }

                        Err(e2) => error_admin_route(e2, &user_req.lang_code),

                    },

                }

            },

            Err(_) => HttpResponse::Found()
                .header("Location", "/{lang}/admin/error?code={code}"
                    .replace("{lang}", &user_req.lang_code)
                    .replace("{code}", CSRF_TOKEN_IS_NOT_A_VALID_UUID)
                )
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}
