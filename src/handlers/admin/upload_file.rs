use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::upload_file::UploadFile;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


pub fn ra_upload_file(
    lang_code: &str,
) -> String {
    "/{lang}/admin/upload_file".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct UploadFileARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for UploadFileARequest {
    fn query(&self) -> &str {
        "SELECT awa_upload_file($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct UploadFileAResponse {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn upload_file(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            UploadFileARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: UploadFileAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = UploadFile {
                    title: &format!(
                        "{a} - {b}",
                        a = t.upload_file,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

