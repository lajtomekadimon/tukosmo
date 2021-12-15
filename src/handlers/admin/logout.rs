use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::handlers::admin::login::ra_login;


pub fn ra_logout(
    lang_code: &str,
) -> String {
    "/{lang}/admin/logout".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LogoutARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for LogoutARequest {
    fn query(&self) -> &str {
        "SELECT awa_logout($1)"
    }
}


pub async fn logout(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id.clone()) {

        Ok(user_req) => {

            match query_db(
                LogoutARequest {
                    req: user_req.clone(),
                }
            ) {

                Ok(_row) => {

                    // Delete auth cookie
                    id.forget();

                    // Redirect to login page
                    HttpResponse::Found()
                        .header("Location", ra_login(&user_req.lang_code))
                        .finish()

                },

                Err(e) => {
                    println!("{}", e);

                    HttpResponse::Found()
                        .header("Location", ra_login(&user_req.lang_code))
                        .finish()
                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}

