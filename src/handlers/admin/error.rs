use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::error::Error;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;
use crate::i18n::error_code_message::error_code_message;


#[derive(Deserialize)]
pub struct GetParamData {
    code: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ErrorARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for ErrorARequest {
    fn query(&self) -> &str {
        "SELECT awa_error($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ErrorAResponse {
    pub data: types::AdminDataDB,
}


pub async fn error(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let error_code = param.code;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            ErrorARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: ErrorAResponse = row.get(0);
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
                        a = "ERROR {code}".replace("{code}", &e.code),
                        b = t.tukosmo_admin_panel,
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
                            // TODO: Redirect to same URL using default lang

                        } else if error_code == ec::USER_NOT_LOGGED_IN {

                            let login_route = "/{lang}/admin/login"
                                .replace("{lang}", &user_req.lang_code);

                            HttpResponse::Found()
                                .header("Location", login_route)
                                .finish()

                        } else {

                            let e_message = "ERROR {code}: {message}"
                                .replace("{code}", error_code)
                                .replace("{message}", &error_code_message(
                                    &error_code,
                                    &user_req.lang_code,
                                ));

                            // TODO: Nice HTML page without db query
                            HttpResponse::Ok().body(e_message)

                        }

                    } else {

                        HttpResponse::Ok().body("UNKNOWN ERROR")  // TODO

                    }

                } else {

                    HttpResponse::Ok().body("UNKNOWN ERROR")  // TODO

                }

            },

        },

        Err(redirect_url) => redirect_url,

    }

}

