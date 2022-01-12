use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::website::{
    blog_get::{
        WgiBlog,
        WgoBlog,
    },
    user_request::user_request,
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
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let user_req = user_request(req, id);

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match query_db(
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
                title: &format!(
                    "{a} - {b}",
                    a = q.data.website_title,
                    b = q.data.website_subtitle,
                ),
                q: &q,
                t: t,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(e) => error_website_route(&e, &user_req.lang_code),

    }

}
