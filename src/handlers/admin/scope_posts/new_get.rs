use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::scope_posts::new::New;


pub fn ra_posts_new(
    lang_code: &str,
) -> String {
    "/{lang}/admin/posts/new".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiPostsNew {
    pub req: types::AdminRequest,
    pub featured_image: Option<i64>,
}

impl QueryFunction for AgiPostsNew {
    fn query(&self) -> &str {
        "SELECT aha_g_posts_new($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoPostsNew {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub featured_image: Option<types::FileDB>,
}


pub async fn new_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiPostsNew {
                req: user_req.clone(),
                featured_image: None,
            },
        ) {

            Ok(row) => {

                let q: AgoPostsNew = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = New {
                    domain: &config.server.domain,
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

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            },

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

