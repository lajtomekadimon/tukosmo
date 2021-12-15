use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use actix_multipart::Multipart;
use postgres_types::{ToSql, FromSql};
use postgres::Error as PgError;

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
//use crate::database::error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::upload_file::{
    UploadFileARequest,
    UploadFileAResponse,
};
use crate::templates::admin::upload_file::UploadFile;
use crate::database::types::AdminRequest;
use crate::files::save_file::save_file;
use crate::handlers::admin::edit_file::ra_edit_file_w_id;


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct UploadFilePostARequest {
    pub req: types::AdminRequest,
    //pub csrf_token: Uuid,
    pub filename: String,
}

impl QueryFunction for UploadFilePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_upload_file_post($1)"
    }
}


pub fn template_upload_file(
    user_req: AdminRequest,
    error: Option<PgError>,
) -> HttpResponse {

    match query_db(
        UploadFileARequest {
            req: user_req.clone(),
        },
    ) {

        Ok(row) => {

            let q: UploadFileAResponse = row.get(0);
            let t = &t(&q.data.lang.code);

            let e = match error {
                Some(e_value) => Some(t_error(e_value, &q.data.lang.code)),
                None => None,
            };

            let html = UploadFile {
                title: &format!(
                    "{a} - {b}",
                    a = t.upload_file,
                    b = t.tukosmo_admin_panel,
                ),
                q: &q,
                t: t,
                error: &e,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(e2) => error_admin_route(e2, &user_req.lang_code),

    }

}


pub async fn upload_file_post(
    req: HttpRequest,
    id: Identity,
    payload: Multipart,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match save_file(payload).await {

            Ok(filename) => match query_db(
                UploadFilePostARequest {
                    req: user_req.clone(),
                    //csrf_token: csrf_token_value,
                    filename: filename.clone(),
                },
            ) {

                Ok(row) => {

                    let file_id: i64 = row.get(0);

                    HttpResponse::Found()
                        .header(
                            "Location",
                            ra_edit_file_w_id(
                                &user_req.lang_code,
                                &file_id,
                            ),
                        )
                        .finish()

                },

                Err(e) => {

                    let filepath = format!("./files/{}", filename);

                    // Delete file when database operation fails
                    match std::fs::remove_file(filepath) {
                        Ok(_) => template_upload_file(
                            user_req,
                            Some(e),
                        ),
                        // TODO: (?)
                        Err(_) => template_upload_file(
                            user_req,
                            Some(e),
                        ),
                    }

                }

            },

            Err(e) => {

                // TODO
                println!("{}", e);
                HttpResponse::Ok().body("Error!!!")

            },

        },

        Err(redirect_url) => redirect_url,

    }

}
