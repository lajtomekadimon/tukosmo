use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::posts::Posts;


#[derive(Deserialize)]
pub struct GetParamData {
    rpp: Option<i64>,
    p: Option<i64>,
    f: Option<String>,
    success: Option<String>,
}


pub fn ra_posts(
    lang_code: &str,
) -> String {
    "/{lang}/admin/posts".replace("{lang}", lang_code)
}

pub fn ra_posts_w_f(
    lang_code: &str,
    f: &str,
) -> String {
    "/{lang}/admin/posts?f={f}"
        .replace("{lang}", lang_code)
        .replace("{f}", f)
}

pub fn ra_posts_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/posts?success=true"
        .replace("{lang}", lang_code)
}

pub fn ra_posts_w_f_wu_rpp_p(
    lang_code: &str,
    f: &str,
) -> String {
    "/{lang}/admin/posts?rpp={rpp}&p={p}&f={f}"
        .replace("{lang}", lang_code)
        .replace("{f}", f)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiPosts {
    pub req: types::AdminRequest,
    pub results_per_page: i64,
    pub page: i64,
    pub filter: String,
}

impl QueryFunction for AgiPosts {
    fn query(&self) -> &str {
        "SELECT aha_g_posts($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoPosts {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub posts: Vec<types::PostDB>,
    pub filter: String,
    pub results_per_page: i64,
    pub page: i64,
    pub total_results: i64,
    pub total_pages: i64,
}


pub async fn posts_get(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            AgiPosts {
                req: user_req.clone(),
                results_per_page: results_per_page,
                page: current_page,
                filter: (param.f).clone().unwrap_or("all".to_string()),
            },
        ) {

            Ok(row) => {

                let q: AgoPosts = row.get(0);
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

