use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use actix_multipart::Multipart;
use serde_json::json;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::files::{
    save_file::save_file,
    file_route::file_route,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    //error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID,
};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiJsonUploadFile {
    pub req: types::AdminRequest,
    //pub csrf_token: Uuid,
}

impl QueryFunction for ApiJsonUploadFile {
    fn query(&self) -> &str {
        "SELECT aha_p_json_upload_file($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApoJsonUploadFile {
    pub data: types::AdminDataDB,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SpiUploadFile {
    pub author_id: i64,
    pub filename: String,
}

impl QueryFunction for SpiUploadFile {
    fn query(&self) -> &str {
        "SELECT as_upload_file($1)"
    }
}


pub fn ra_json_upload_file(
    lang_code: &str,
) -> String {
    "/{lang}/admin/json/upload_file"
        .replace("{lang}", lang_code)
}


pub async fn upload_file_post(
    req: HttpRequest,
    id: Identity,
    payload: Multipart,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            ApiJsonUploadFile {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: ApoJsonUploadFile = row.get(0);

                match save_file(payload).await {

                    Ok(filename) => match query_db(
                        SpiUploadFile {
                            author_id: q.data.userd.id,
                            filename: filename.clone(),
                        },
                    ){

                        Ok(row2) => {

                            let file_id: i64 = row2.get(0);

                            let body = json!({
                                "success": true,
                                "id": file_id,
                                "filename": filename.clone(),
                                "url": file_route(&filename.clone()),
                            });
                            HttpResponse::Ok()
                                .content_type("application/json")
                                .body(body.to_string())

                        },

                        Err(_e) => {

                            // TODO: Delete file!!!

                            let body = json!({
                                "success": false,
                                // TODO: Show error
                            });
                            HttpResponse::Ok()
                                .content_type("application/json")
                                .body(body.to_string())

                        },


                    },

                    Err(e) => {

                        // TODO
                        println!("{}", e);

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

