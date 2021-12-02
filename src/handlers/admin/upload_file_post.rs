use actix_web::{web, HttpRequest, HttpResponse, Responder, Error};
use actix_identity::Identity;
use actix_multipart::Multipart;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;
use std::io::Write;
use futures_util::TryStreamExt as _;  // in save_file(), but not sure if works
use postgres::Error as PgError;
use std::path::Path;
use std::ffi::OsStr;

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


async fn save_file(
    mut payload: Multipart,
) -> Result<String, Error> {

    let mut rvalue: Result<String, Error> = Ok("".to_string());

    // Iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        // A multipart/form-data stream has to contain `content_disposition`
        let content_disposition = field
            .content_disposition()
            .ok_or_else(|| HttpResponse::BadRequest().finish())?;
            // I think this HttpResponse should be a different value (type)...

        let filename = content_disposition.get_filename().map_or_else(
            || Uuid::new_v4().to_string(),
            |f| sanitize_filename::sanitize(f),
        );
        // TODO: Return the original filename too so UUID is not used.
        
        // If there's a file
        if filename != "" {
            let uuid_text = Uuid::new_v4().to_string();

            let filename_value = match Path::new(&filename)
                .extension()
                .and_then(OsStr::to_str)
            {
                Some(filename_ext) => format!(
                    "{}.{}",
                    uuid_text,
                    filename_ext.to_lowercase(),
                ),
                None => uuid_text,
            };

            let filepath = format!("./files/{}", filename_value);

            // File::create is blocking operation, use threadpool
            let mut f = web::block(|| std::fs::File::create(filepath)).await?;

            // Field in turn is stream of *Bytes* object
            while let Some(chunk) = field.try_next().await? {
                // filesystem operations are blocking, we have to use threadpool
                f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
            }

            rvalue = Ok(filename_value);
        }
    }

    rvalue

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

                    let redirect_route = "/{lang}/admin/edit_file?id={id}"
                        .replace("{lang}", &user_req.lang_code)
                        .replace("{id}", &file_id.to_string());

                    HttpResponse::Found()
                        .header("Location", redirect_route)
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
