use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use serde_json::json;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    //error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID,
    error_codes::UUID_IS_NOT_VALID,
};
use crate::i18n::{
    t_error::t_error,
    error_code_message::error_code_message,
};


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
    pub file_title: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiJsonEditFile {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub id: i64,
    pub file_title: String,
}

impl QueryFunction for ApiJsonEditFile {
    fn query(&self) -> &str {
        "SELECT aha_p_json_edit_file($1)"
    }
}


pub fn ra_json_edit_file(
    lang_code: &str,
) -> String {
    "/{lang}/admin/json/edit_file"
        .replace("{lang}", lang_code)
}


pub async fn edit_file_post(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let file_id = (form.id).clone();
                let file_title_value = (form.file_title).clone();

                match query_db(
                    &config,
                    ApiJsonEditFile {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        id: file_id.clone(),
                        file_title: file_title_value.clone(),
                    },
                ) {

                    Ok(_row) => {

                        let body = json!({
                            "success": true,
                        });
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .body(body.to_string())

                    },

                    Err(e) => {
                        let error_v = &t_error(&e, &user_req.lang_code);

                        let body = json!({
                            "success": false,
                            "error_code": error_v.code,
                            "error_message": error_v.message,
                        });
                        HttpResponse::Ok()
                            .content_type("application/json")
                            .body(body.to_string())

                    },

                }

            },

            Err(_e) => {
                let body = json!({
                    "success": false,
                    "error_code": UUID_IS_NOT_VALID,
                    "error_message": &error_code_message(
                        UUID_IS_NOT_VALID,
                        &user_req.lang_code,
                    ),
                });
                HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body.to_string())
            },

        },

        Err(_redirect_url) => {
            let body = json!({
                "success": false,
                // TODO: Show error
            });
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body.to_string())
        },

    }

}
