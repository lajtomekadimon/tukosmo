use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::website::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::t_error::ErrorDB;
use crate::i18n::error_code_message::error_code_message;
use crate::templates::website::error::Error;
use crate::database::types;
use crate::database::error_codes as ec;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    code: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ErrorWRequest {
    pub req: types::WebsiteRequest,
    pub code: String,
}

impl QueryFunction for ErrorWRequest {
    fn query(&self) -> &str {
        "SELECT aww_error($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ErrorWResponse {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn error(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let error_code = param.code;

    let user_req = user_request(req, id);

    match query_db(
        ErrorWRequest {
            req: user_req,
            code: error_code.clone(),
        },
    ) {

        Ok(row) => {

            let q: ErrorWResponse = row.get(0);
            let t = &t(&q.data.lang.code);

            let e: ErrorDB = ErrorDB {
                code: error_code.clone(),
                message: error_code_message(
                    &error_code,
                    &q.data.lang.code,
                ).to_string(),
            };

            let html = Error {
                title: &format!(
                    "{a} - {b}",
                    a = t.error,
                    b = q.data.website_title,
                ),
                q: &q,
                t: t,
                e: &e,
            };

            HttpResponse::Ok().body(html.to_string())

        },

        Err(e) => {
            println!("{}", e);  // for debugging

            if let Some(dberror) = e.as_db_error() {

                let error_message = dberror.message();

                if &error_message[..8] == "TUKOSMO:" {

                    // Extract X from "TUKOSMO:X"
                    let error_code = &error_message[8..];

                    if error_code == ec::WRONG_LANG_CODE {

                        HttpResponse::Ok().body("WRONG URL LANGUAGE CODE")
                        // TODO: Redirect to same URL using default language

                    } else {

                        HttpResponse::Ok().body("UNKNOWN ERROR")  // TODO

                    }

                } else {

                    HttpResponse::Ok().body("UNKNOWN ERROR")  // TODO

                }

            } else {

                HttpResponse::Ok().body("UNKNOWN ERROR")  // TODO

            }
        },

    }

}

