use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::i18n::t::t;
use crate::i18n::error_website_route::error_website_route;
use crate::handlers::website::user_request::user_request;
use crate::templates::website::blog_post::BlogPost;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


pub fn rw_blog_post(
    lang_code: &str,
    permalink: &str,
) -> String {
    "/{lang}/blog/{permalink}"
        .replace("{lang}", lang_code)
        .replace("{permalink}", permalink)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct BlogPostWRequest {
    pub req: types::WebsiteRequest,
    pub permalink: String,
}

impl QueryFunction for BlogPostWRequest {
    fn query(&self) -> &str {
        "SELECT aww_blog_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct BlogPostWResponse {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
    pub post: types::PostDB,
}


pub async fn blog_post(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req.clone(), id);

    let permalink_value: String = req.match_info()
        .get("permalink").unwrap().parse().unwrap();

    match query_db(
        BlogPostWRequest {
            req: user_req.clone(),
            permalink: permalink_value,
        },
    ) {

        Ok(row) => {

            let q: BlogPostWResponse = row.get(0);
            let t = &t(&q.data.lang.code);

            let html = BlogPost {
                title: &format!(
                    "{a} - {b}",
                    a = q.post.title,
                    b = q.data.website_title,
                ),
                q: &q,
                t: t,
            };

            HttpResponse::Ok().body(html.to_string())

        },

        Err(e) => error_website_route(e, &user_req.lang_code),

    }

}
