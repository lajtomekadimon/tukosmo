use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::website::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_website_route::error_website_route,
};
use crate::templates::website::scope_blog::post::Post;


pub fn rw_blog_post(
    lang_code: &str,
    permalink: &str,
) -> String {
    "/{lang}/blog/{permalink}"
        .replace("{lang}", lang_code)
        .replace("{permalink}", permalink)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgiBlogPost {
    pub req: types::WebsiteRequest,
    pub permalink: String,
}

impl QueryFunction for WgiBlogPost {
    fn query(&self) -> &str {
        "SELECT ahw_g_blog_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgoBlogPost {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
    pub post: types::PostDB,
}


pub async fn post_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req.clone(), id);

    let permalink_value: String = req.match_info()
        .get("permalink").unwrap().parse().unwrap();

    match query_db(
        &config,
        WgiBlogPost {
            req: user_req.clone(),
            permalink: permalink_value,
        },
    ) {

        Ok(row) => {

            let q: WgoBlogPost = row.get(0);
            let t = &t(&q.data.lang.code);

            let html = Post {
                domain: &config.server.domain,
                title: &format!(
                    "{a} - {b}",
                    a = q.post.title,
                    b = q.data.website_title,
                ),
                q: &q,
                t: t,
            };

            HttpResponse::Ok()
                .content_type("text/html; charset=UTF-8")
                .body(html.to_string())

        },

        Err(e) => error_website_route(&e, &user_req.lang_code),

    }

}
