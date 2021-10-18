use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::posts::Posts;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    rpp: Option<i64>,
    p: Option<i64>,
    f: Option<String>,
    success: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PostsARequest {
    pub req: types::AdminRequest,
    pub results_per_page: i64,
    pub page: i64,
    pub filter: String,
}

impl QueryFunction for PostsARequest {
    fn query(&self) -> &str {
        "SELECT awa_posts($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PostsAResponse {
    pub data: types::AdminDataDB,
    pub posts: Vec<types::PostDB>,
    pub filter: String,
    pub results_per_page: i64,
    pub page: i64,
    pub total_results: i64,
    pub total_pages: i64,
}


pub async fn posts(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            PostsARequest {
                req: user_req,
                results_per_page: results_per_page,
                page: current_page,
                filter: (param.f).clone().unwrap_or("all".to_string()),
            },
        ) {

            Ok(row) => {

                let q: PostsAResponse = row.get(0);

                let html = Posts {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(if q.filter == "drafts" {
                            "Draft posts"
                        } else if q.filter == "published" {
                            "Published posts"
                        } else if q.filter == "untranslated" {
                            "Untranslated posts"
                        } else if q.filter == "deleted" {
                            "Deleted posts"
                        } else {
                            "Posts"
                        }, &q.data.lang.code),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    success: match param.success {
                        Some(_) => &true,
                        None => &false,
                    },
                };

                HttpResponse::Ok().body(html.to_string())

            }

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

