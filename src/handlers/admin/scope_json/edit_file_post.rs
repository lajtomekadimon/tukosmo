use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use serde_json::json;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::{
    user_request::user_request,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    //error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID,
};
use crate::i18n::t_error::t_error;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
    pub filename: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiJsonEditFile {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub id: i64,
    pub filename: String,
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
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let file_id = (form.id).clone();
                let filename_value = (form.filename).clone();

                match query_db(
                    ApiJsonEditFile {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        id: file_id.clone(),
                        filename: filename_value.clone(),
                    },
                ) {

                    Ok(row) => {

                        let ofilename: String = row.get(0);

                        let filepath = format!("./files/{}", ofilename);
                        let nfilepath = format!("./files/{}", filename_value);

                        let body = json!({
                            "success": true,
                        });

                        // TODO: What if file couldn't be renamed?
                        match std::fs::rename(filepath, nfilepath) {
                            Ok(_) => HttpResponse::Ok()
                                .content_type("application/json")
                                .body(body.to_string()),
                            Err(_) => HttpResponse::Ok()
                                .content_type("application/json")
                                .body(body.to_string()),
                        }

                    },

                    Err(e) => {

                        // TODO
                        println!("{}", e);
                        println!("{}", t_error(e, "en").message);

                        let body = json!({
                            "success": false,
                            // TODO: Show error
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
                    // TODO: Show error
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
