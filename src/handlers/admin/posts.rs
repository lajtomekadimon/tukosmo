use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
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
    pub routes: Vec<types::RouteDB>,
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
                req: user_req.clone(),
                results_per_page: results_per_page,
                page: current_page,
                filter: (param.f).clone().unwrap_or("all".to_string()),
            },
        ) {

            Ok(row) => {

                let q: PostsAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Posts {
                    title: &format!(
                        "{a} - {b}",
                        a = if q.filter == "drafts" {
                            t.draft_posts
                        } else if q.filter == "published" {
                            t.published_posts
                        } else if q.filter == "untranslated" {
                            t.untranslated_posts
                        } else if q.filter == "deleted" {
                            t.deleted_posts
                        } else {
                            t.posts
                        },
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: match param.success {
                        Some(_) => &true,
                        None => &false,
                    },
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

