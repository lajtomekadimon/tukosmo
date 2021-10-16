use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::website::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::website::error::Error;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ErrorWRequest {
    pub req: types::WebsiteRequest,
}

impl QueryFunction for ErrorWRequest {
    fn query(&self) -> &str {
        "SELECT aww_error($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ErrorWResponse {
    pub data: types::WebsiteDataDB,
}


pub async fn error(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req, id);

    match query_db(
        ErrorWRequest {
            req: user_req,
        },
    ) {

        Ok(row) => {

            let q: ErrorWResponse = row.get(0);

            let html = Error {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Error", &q.data.lang.code),
                    b = "MyExample"
                ),
                q: &q,
            };

            HttpResponse::Ok().body(html.to_string())

        },

        Err(e) => {
            println!("{}", e);
            HttpResponse::Ok().body("UNKNOWN ERROR")  // TODO
        },

    }

}

