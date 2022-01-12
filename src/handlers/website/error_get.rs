use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::DEFAULT_LANG;
use crate::handlers::website::user_request::user_request;
use crate::database::{
    types,
    error_codes as ec,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    t_error::ErrorDB,
    error_code_message::error_code_message,
};
use crate::templates::website::error::Error;


#[derive(Deserialize)]
pub struct GetParamData {
    code: String,
}


pub fn rw_error_w_code(
    lang_code: &str,
    code: &str,
) -> String {
    "/{lang}/error?code={code}"
        .replace("{lang}", lang_code)
        .replace("{code}", code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgiError {
    pub req: types::WebsiteRequest,
    pub code: String,
}

impl QueryFunction for WgiError {
    fn query(&self) -> &str {
        "SELECT ahw_g_error($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgoError {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn error_get(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let error_code = param.code;

    let user_req = user_request(req, id);

    match query_db(
        WgiError {
            req: user_req,
            code: error_code.clone(),
        },
    ) {

        Ok(row) => {

            let q: WgoError = row.get(0);
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
            if let Some(dberror) = e.as_db_error() {

                let error_message = dberror.message();

                if &error_message[..8] == "TUKOSMO:" {

                    // Extract X from "TUKOSMO:X"
                    let error_code = &error_message[8..];

                    if error_code == ec::WRONG_LANG_CODE {

                        // Redirect to same URL using default language
                        HttpResponse::Found()
                            .header(
                                "Location",
                                rw_error_w_code(
                                    DEFAULT_LANG,
                                    error_code,
                                ),
                            )
                            .finish()

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

