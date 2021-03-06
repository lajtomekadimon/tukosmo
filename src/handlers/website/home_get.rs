use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::config::global::Config;
use crate::handlers::website::{
    blog_get::{
        WgiBlog,
        WgoBlog,
    },
    user_request::user_request,
    http_data_request::http_data_request,
};
use crate::database::query_db::query_db;
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


pub fn rw_home(
    lang_code: &str,
) -> String {
    "/{lang}".replace("{lang}", lang_code)
}

pub async fn home_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let user_req = user_request(req.clone(), id);

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match query_db(
        &config,
        WgiBlog {
            req: user_req.clone(),
            http_data: http_data_request(req.clone()),
            results_per_page: results_per_page,
            page: current_page,
        },
    ).await {

        Ok(row) => {

            let q: WgoBlog = row.get(0);
            let t = &t(&q.data.lang.code);

            let html = Blog {
                domain: &config.server.domain,
                codename: &codename,
                title: &format!(
                    "{a} - {b}",
                    a = q.data.website_title,
                    b = q.data.website_subtitle,
                ),
                q: &q,
                t: t,
            };

            HttpResponse::Ok()
                .content_type("text/html; charset=UTF-8")
                .body(html.to_string())

        }

        Err(e) => error_website_route(&e, &user_req.lang_code),

    }

}
