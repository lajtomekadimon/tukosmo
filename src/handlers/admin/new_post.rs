use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::new_post::NewPost;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewPostARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for NewPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_new_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewPostAResponse {
    pub data: types::AdminDataDB,
}


pub async fn new_post(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            NewPostARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: NewPostAResponse = row.get(0);

                let html = NewPost {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("New post", &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

