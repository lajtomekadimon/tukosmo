use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
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
use crate::templates::website::blog::Blog;


#[derive(Deserialize)]
pub struct GetParamData {
    rpp: Option<i64>,
    p: Option<i64>,
}


/*
pub fn rw_blog(
    lang_code: &str,
) -> String {
    "/{lang}/blog".replace("{lang}", lang_code)
}
*/

pub fn rw_blog_wu_rpp_p(
    lang_code: &str,
) -> String {
    "/{lang}/blog?rpp={rpp}&p={p}"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgiBlog {
    pub req: types::WebsiteRequest,
    pub results_per_page: i64,
    pub page: i64,
}

impl QueryFunction for WgiBlog {
    fn query(&self) -> &str {
        "SELECT ahw_g_blog($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgoBlog {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
    pub posts: Vec<types::PostDB>,
    pub results_per_page: i64,
    pub page: i64,
    pub total_results: i64,
    pub total_pages: i64,
}


pub async fn blog_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let user_req = user_request(req, id);

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match query_db(
        &config,
        WgiBlog {
            req: user_req.clone(),
            results_per_page: results_per_page,
            page: current_page,
        },
    ) {

        Ok(row) => {

            let q: WgoBlog = row.get(0);
            let t = &t(&q.data.lang.code);

            let html = Blog {
                domain: &config.server.domain,
                title: &format!(
                    "{a} - {b}",
                    a = t.blog,
                    b = q.data.website_title,
                ),
                q: &q,
                t: t,
            };

            HttpResponse::Ok().body(html.to_string())

        },

        Err(e) => error_website_route(&e, &user_req.lang_code),

    }

}

