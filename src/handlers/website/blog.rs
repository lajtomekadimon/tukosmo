use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
//use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::website::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::website::blog::Blog;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct BlogWRequest {
    pub req: types::WebsiteRequest,
    //pub results_per_page: i64,
    //pub page: i64,
}

impl QueryFunction for BlogWRequest {
    fn query(&self) -> &str {
        "SELECT aww_blog($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct BlogWResponse {
    pub data: types::WebsiteDataDB,
    pub posts: Vec<types::PostDB>,
    //pub results_per_page: i64,
    //pub page: i64,
    pub total_results: i64,
    //pub total_pages: i64,
}


pub async fn blog(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req, id);

    match query_db(
        BlogWRequest {
            req: user_req,
            //results_per_page: results_per_page,
            //page: current_page,
        },
    ) {

        Ok(row) => {

            let q: BlogWResponse = row.get(0);

            let html = Blog {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Blog", &q.data.lang.code),
                    b = "MyExample"
                ),
                q: &q,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(e) => {
            println!("{}", e);
            panic!("HAAAA");  // TODO
        },

    }

}

