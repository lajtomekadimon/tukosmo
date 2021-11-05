use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::i18n::t::t;
use crate::i18n::error_website_route::error_website_route;
use crate::handlers::website::user_request::user_request;
use crate::templates::website::page::Page;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PageWRequest {
    pub req: types::WebsiteRequest,
}

impl QueryFunction for PageWRequest {
    fn query(&self) -> &str {
        "SELECT aww_page($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PageWResponse {
    pub data: types::WebsiteDataDB,
}


pub async fn page(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req, id);

    match query_db(
        PageWRequest {
            req: user_req.clone(),
        },
    ) {

        Ok(row) => {

            let q: PageWResponse = row.get(0);
            let t = &t(&q.data.lang.code);

            let html = Page {
                title: &format!(
                    "{a} - {b}",
                    a = "[page title]",
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

