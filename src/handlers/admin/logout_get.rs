use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::{
    user_request::user_request,
    login_get::ra_login,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};


pub fn ra_logout(
    lang_code: &str,
) -> String {
    "/{lang}/admin/logout".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiLogout {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiLogout {
    fn query(&self) -> &str {
        "SELECT aha_g_logout($1)"
    }
}


pub async fn logout_get(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id.clone()) {

        Ok(user_req) => {

            match query_db(
                AgiLogout {
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

                Err(_e) => {
                    HttpResponse::Found()
                        .header("Location", ra_login(&user_req.lang_code))
                        .finish()
                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}

