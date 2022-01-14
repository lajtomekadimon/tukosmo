use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
    login_get::ra_login,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    error_codes as ec,
};
use crate::i18n::{
    t::t,
    t_error::ErrorDB,
    error_code_message::error_code_message,
};
use crate::templates::admin::error::Error;


#[derive(Deserialize)]
pub struct GetParamData {
    code: String,
}


pub fn ra_error_w_code(
    lang_code: &str,
    code: &str,
) -> String {
    "/{lang}/admin/error?code={code}"
        .replace("{lang}", lang_code)
        .replace("{code}", code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiError {
    pub req: types::AdminRequest,
    pub code: String,
}

impl QueryFunction for AgiError {
    fn query(&self) -> &str {
        "SELECT aha_g_error($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoError {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn error_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let error_code = param.code;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiError {
                req: user_req.clone(),
                code: error_code.clone(),
            },
        ) {

            Ok(row) => {

                let q: AgoError = row.get(0);
                let t = &t(&q.data.lang.code);

                let e: ErrorDB = ErrorDB {
                    code: error_code.clone(),
                    message: error_code_message(
                        &error_code,
                        &q.data.lang.code,
                    ).to_string(),
                };

                let html = Error {
                    domain: &config.server.domain,
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

                            HttpResponse::Found()
                                .header(
                                    "Location",
                                    ra_login(&user_req.lang_code),
                                )
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

