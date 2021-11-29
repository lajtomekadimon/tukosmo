use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::edit_file::EditFile;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditFileARequest {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for EditFileARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_file($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditFileAResponse {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub file_data: types::FileDB,
}


pub async fn edit_file(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let file_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            EditFileARequest {
                req: user_req.clone(),
                id: file_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: EditFileAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = EditFile {
                    title: &format!(
                        "{a} - {b}",
                        a = t.edit_file_w_name
                            .replace("{name}", &q.file_data.name),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

