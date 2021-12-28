use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::new_post::NewPost;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


pub fn ra_new_post(
    lang_code: &str,
) -> String {
    "/{lang}/admin/new_post".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewPostARequest {
    pub req: types::AdminRequest,
    pub featured_image: Option<i64>,
}

impl QueryFunction for NewPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_new_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewPostAResponse {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub featured_image: Option<types::FileDB>,
}


pub async fn new_post(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            NewPostARequest {
                req: user_req.clone(),
                featured_image: None,
            },
        ) {

            Ok(row) => {

                let q: NewPostAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = NewPost {
                    title: &format!(
                        "{a} - {b}",
                        a = t.new_post,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
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

