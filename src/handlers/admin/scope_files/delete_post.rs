use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
    scope_files::delete_get::{
        AgiFilesDelete,
        AgoFilesDelete,
    },
    files_get::ra_files_success,
    error_get::ra_error_w_code,
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
use crate::templates::admin::scope_files::delete::Delete;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiFilesDelete {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub id: i64,
}

impl QueryFunction for ApiFilesDelete {
    fn query(&self) -> &str {
        "SELECT aha_p_files_delete($1)"
    }
}


pub async fn delete_post(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let file_id = (form.id).clone();

                match query_db(
                    &config,
                    ApiFilesDelete {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        id: file_id.clone(),
                    },
                ).await {

                    Ok(row) => {

                        let filename: String = row.get(0);

                        let filepath = format!("./files/{}", filename);

                        // Delete file when database operation fails
                        match std::fs::remove_file(filepath) {
                            Ok(_) => {
                                HttpResponse::Found()
                                    .append_header((
                                        "Location",
                                        ra_files_success(&user_req.lang_code),
                                    ))
                                    .finish()
                            },
                            // TODO: (?)
                            Err(_) => {
                                HttpResponse::Found()
                                    .append_header((
                                        "Location",
                                        ra_files_success(&user_req.lang_code),
                                    ))
                                    .finish()
                            },
                        }

                    },

                    Err(e) => match query_db(
                        &config,
                        AgiFilesDelete {
                            req: user_req.clone(),
                            id: file_id.clone(),
                        },
                    ).await {

                        Ok(row) => {

                            let q: AgoFilesDelete = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = Delete {
                                domain: &config.server.domain,
                                codename: &codename,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.delete_file_w_name
                                        .replace("{name}", &q.file_data.name),
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                error: &Some(t_error(&e, &q.data.lang.code)),
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
                .append_header((
                    "Location",
                    ra_error_w_code(
                        &user_req.lang_code,
                        CSRF_TOKEN_IS_NOT_A_VALID_UUID,
                    ),
                ))
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}
