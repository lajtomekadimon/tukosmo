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
use crate::templates::website::page::Page;


/*
pub fn rw_page(
    lang_code: &str,
) -> String {
    "/{lang}/page".replace("{lang}", lang_code)
}
*/

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgiPage {
    pub req: types::WebsiteRequest,
}

impl QueryFunction for WgiPage {
    fn query(&self) -> &str {
        "SELECT ahw__page($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgoPage {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn page_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req, id);

    match query_db(
        &config,
        WgiPage {
            req: user_req.clone(),
        },
    ) {

        Ok(row) => {

            let q: WgoPage = row.get(0);
            let t = &t(&q.data.lang.code);

            let html = Page {
                domain: &config.server.domain,
                codename: &codename,
                title: &format!(
                    "{a} - {b}",
                    a = "[page title]",
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

