use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::scope_posts::edit::Edit;


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


pub fn ra_posts_edit_w_id(
    lang_code: &str,
    id: &i64,
) -> String {
    "/{lang}/admin/posts/edit?id={id}"
        .replace("{lang}", lang_code)
        .replace("{id}", &id.to_string())
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiPostsEdit {
    pub req: types::AdminRequest,
    pub post: i64,
    pub featured_image: Option<i64>,
    pub first_request: bool,
    pub tags_added: Vec<i64>,
}

impl QueryFunction for AgiPostsEdit {
    fn query(&self) -> &str {
        "SELECT aha_g_posts_edit($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoPostsEdit {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub id: i64,
    pub post: Option<types::PostDB>,
    pub featured_image: Option<types::FileDB>,
    pub tags: Vec<types::TagDB>,
    pub tags_added: Vec<types::TagDB>,
}


pub async fn edit_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let post_id = param.id;

    let empty_tags_added: Vec<i64> = Vec::new();

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiPostsEdit {
                req: user_req.clone(),
                post: post_id.clone(),
                featured_image: None,
                first_request: true,
                tags_added: empty_tags_added,
            },
        ) {

            Ok(row) => {

                let q: AgoPostsEdit = row.get(0);
                let t = &t(&q.data.lang.code);

                let post_id_str = &post_id.to_string();
                let title_tab_value = if let Some(post) = &q.post {
                    &post.title
                } else {
                    post_id_str
                };

                let html = Edit {
                    domain: &config.server.domain,
                    codename: &codename,
                    title: &format!(
                        "{a} - {b}",
                        a = t.edit_post_w_title
                            .replace("{title}", title_tab_value),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            }

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

