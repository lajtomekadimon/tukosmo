use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::delete_post::DeletePost;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeletePostARequest {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for DeletePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_delete_post($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeletePostAResponse {
    pub data: types::AdminDataDB,
    pub post: types::PostDB,
}


pub async fn delete_post(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let post_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            DeletePostARequest {
                req: user_req.clone(),
                id: post_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: DeletePostAResponse = row.get(0);

                let html = DeletePost {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Delete post: '{title}'",
                            &q.data.lang.code
                        ).replace("{title}", &q.post.title),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

