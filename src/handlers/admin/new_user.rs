use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::new_user::NewUser;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewUserARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for NewUserARequest {
    fn query(&self) -> &str {
        "SELECT awa_new_user($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewUserAResponse {
    pub data: types::AdminDataDB,
}


pub async fn new_user(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            NewUserARequest {
                req: user_req,
            },
        ) {

            Ok(row) => {

                let q: NewUserAResponse = row.get(0);

                let html = NewUser {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("New user", &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => {
                println!("{}", e);
                HttpResponse::Found()
                    .header("Location", "/")  // TODO
                    .finish()
            },

        },

        Err(redirect_url) => redirect_url,

    }

}

