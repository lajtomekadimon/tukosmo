use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct FormData {
    id: i64,
    title: String,
    description: String,
    body: String,
    permalink: String,
    draft: Option<String>,
    deleted: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditPostPostARequest {
    pub req: types::AdminRequest,
    pub post: types::PostDB,
}

impl QueryFunction for EditPostPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_post_post($1)"
    }
}


pub async fn edit_post_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {

            let post_id = (form.id).clone();
            let title_value = (form.title).clone();
            let description_value = (form.description).clone();
            let body_value = (form.body).clone();
            let permalink_value = (form.permalink).clone();
            let is_draft: bool = match (form.draft).clone() {
                Some(_) => true,
                None => false,
            };
            let is_deleted: bool = match (form.deleted).clone() {
                Some(_) => true,
                None => false,
            };

            match query_db(
                EditPostPostARequest {
                    req: user_req.clone(),
                    post: types::PostDB {
                        id: post_id,
                        trans_id: 0,
                        lang: 0,
                        title: title_value,
                        description: description_value,
                        body: body_value,
                        permalink: permalink_value,
                        author: 0,
                        author_name: "".to_string(),
                        translator: 0,
                        translator_name: "".to_string(),
                        date: "".to_string(),
                        date_trans: "".to_string(),
                        draft: is_draft,
                        deleted: is_deleted,
                    },
                },
            ) {

                Ok(_row) => {
                    let redirect_route = "/{lang}/admin/posts"
                        .replace("{lang}", &user_req.lang_code);

                    HttpResponse::Found()
                        .header("Location", redirect_route)
                        .finish()
                },
                Err(e) => {
                    println!("{}", e);

                    let redirect_route = "/{lang}/admin/edit_post?id={id}"
                        .replace("{lang}", &user_req.lang_code)
                        .replace("{id}", &post_id.to_string());
                    // TODO: Show what failed in the template!

                    HttpResponse::Found()
                        .header("Location", redirect_route)
                        .finish()
                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}
