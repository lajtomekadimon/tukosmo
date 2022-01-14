use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
    scope_files::edit_get::{
        AgiFilesEdit,
        AgoFilesEdit,
    },
    error_get::ra_error_w_code,
    files_get::ra_files_success,
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
use crate::templates::admin::scope_files::edit::Edit;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
    pub filename: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiFilesEdit {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub id: i64,
    pub filename: String,
}

impl QueryFunction for ApiFilesEdit {
    fn query(&self) -> &str {
        "SELECT aha_p_files_edit($1)"
    }
}


pub async fn edit_post(
    config: web::Data<Config>,
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
                    &config,
                    ApiFilesEdit {
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

                        // TODO: What if file couldn't be renamed?
                        match std::fs::rename(filepath, nfilepath) {
                            Ok(_) => HttpResponse::Found()
                                .header(
                                    "Location",
                                    ra_files_success(&user_req.lang_code),
                                )
                                .finish(),
                            Err(_) => HttpResponse::Found()
                                .header(
                                    "Location",
                                    ra_files_success(&user_req.lang_code),
                                )
                                .finish(),
                        }

                    },

                    Err(e) => match query_db(
                        &config,
                        AgiFilesEdit {
                            req: user_req.clone(),
                            id: file_id.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: AgoFilesEdit = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = Edit {
                                domain: &config.server.domain,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.edit_file_w_name
                                        .replace("{name}", &q.file_data.name),
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                error: &Some(
                                    t_error(&e, &q.data.lang.code),
                                ),
                                form: &Some(form),
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
