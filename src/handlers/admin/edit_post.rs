use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::edit_post::EditPost;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditPostARequest {
    pub req: types::AdminRequest,
    pub post: i64,
}

impl QueryFunction for EditPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditPostAResponse {
    pub data: types::AdminDataDB,
    pub csrf_token: String,
    pub post: Option<types::PostDB>,
}


pub async fn edit_post(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let post_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            EditPostARequest {
                req: user_req.clone(),
                post: post_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: EditPostAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                if let Some(ref post) = q.post {
                    let html = EditPost {
                        title: &format!(
                            "{a} - {b}",
                            a = t.edit_post_w_title
                                .replace("{title}", &post.title),
                            b = t.tukosmo_admin_panel,
                        ),
                        q: &q,
                        t: t,
                        error: &None,
                        form: &None,
                    };

                    HttpResponse::Ok().body(html.to_string())
                } else {
                    let html = EditPost {
                        title: &format!(
                            "{a} - {b}",
                            a = t.edit_post_w_title
                                .replace("{title}", &post_id.to_string()),
                            b = t.tukosmo_admin_panel,
                        ),
                        q: &EditPostAResponse {
                            data: q.data.clone(),
                            csrf_token: q.csrf_token.clone(),
                            post: Some(
                                types::PostDB{
                                    id: post_id,
                                    trans_id: 0,
                                    lang: q.data.lang.clone(),
                                    title: "".to_string(),
                                    description: "".to_string(),
                                    body: "".to_string(),
                                    permalink: "".to_string(),
                                    author: 0,
                                    author_name: "".to_string(),
                                    translator: q.data.userd.id,
                                    translator_name: "".to_string(),
                                    date: "".to_string(),
                                    date_trans: "".to_string(),
                                    draft: false,
                                    deleted: false,
                                }
                            ),
                        },
                        t: t,
                        error: &None,
                        form: &None,
                    };

                    HttpResponse::Ok().body(html.to_string())
                }

            }

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

