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
use crate::handlers::admin::delete_language::{
    DeleteLanguageARequest,
    DeleteLanguageAResponse,
};
use crate::templates::admin::delete_language::DeleteLanguage;
use crate::handlers::admin::error::ra_error_w_code;
use crate::handlers::admin::languages::ra_languages_success;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteLanguagePostARequest {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub id: i64,
}

impl QueryFunction for DeleteLanguagePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_delete_language_post($1)"
    }
}


pub async fn delete_language_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let lang_id = (form.id).clone();

                match query_db(
                    DeleteLanguagePostARequest {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        id: lang_id.clone(),
                    },
                ) {

                    Ok(_row) => {

                        HttpResponse::Found()
                            .header(
                                "Location",
                                ra_languages_success(&user_req.lang_code),
                            )
                            .finish()

                    },

                    Err(e) => match query_db(
                        DeleteLanguageARequest {
                            req: user_req.clone(),
                            id: lang_id.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: DeleteLanguageAResponse = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = DeleteLanguage {
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.delete_language_w_name
                                        .replace("{name}", &q.lang.name),
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
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
