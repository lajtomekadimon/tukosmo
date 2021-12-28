use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::database::error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::edit_post::{
    EditPostARequest,
    EditPostAResponse,
};
use crate::templates::admin::edit_post::EditPost;
use crate::handlers::admin::error::ra_error_w_code;
use crate::handlers::admin::posts::ra_posts_success;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
    pub title: String,
    pub featured_image: i64,
    pub description: String,
    pub body: String,
    pub permalink: String,
    pub draft: Option<String>,
    pub deleted: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditPostPostARequest {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub post: types::PostDB,
    pub featured_image: i64,
}

impl QueryFunction for EditPostPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_post_post($1)"
    }
}


pub async fn edit_post_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

            Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let post_id = (form.id).clone();
                let title_value = (form.title).clone();
                let featured_image_id = (form.featured_image).clone();
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
                        csrf_token: csrf_token_value,
                        post: types::PostDB {
                            id: post_id,
                            featured_image: None,
                            trans_id: 0,
                            lang: types::LanguageDB {
                                id: 0,
                                code: "".to_string(),
                                name: "".to_string(),
                                original_name: "".to_string(),
                                date: "".to_string(),
                                has_all_names: false,
                            },
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
                        featured_image: featured_image_id,
                    },
                ) {

                    Ok(_row) => {

                        HttpResponse::Found()
                            .header(
                                "Location",
                                ra_posts_success(&user_req.lang_code),
                            )
                            .finish()

                    },

                    Err(e) => match query_db(
                        EditPostARequest {
                            req: user_req.clone(),
                            post: post_id.clone(),
                            featured_image: if featured_image_id == 0 {
                                None
                            } else { Some(featured_image_id) },
                            first_request: false,
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
                                    error: &Some(
                                        t_error(e, &q.data.lang.code),
                                    ),
                                    form: &Some(form),
                                };

                                HttpResponse::Ok().body(html.to_string())
                            } else {
                                let html = EditPost {
                                    title: &format!(
                                        "{a} - {b}",
                                        a = t.edit_post_w_title
                                            .replace(
                                                "{title}",
                                                &post_id.to_string(),
                                            ),
                                        b = t.tukosmo_admin_panel,
                                    ),
                                    q: &EditPostAResponse {
                                        data: q.data.clone(),
                                        routes: q.routes.clone(),
                                        csrf_token: q.csrf_token.clone(),
                                        post: Some(
                                            types::PostDB{
                                                id: post_id,
                                                featured_image: None,
                                                trans_id: 0,
                                                lang: types::LanguageDB {
                                                    id: 0,
                                                    code: "".to_string(),
                                                    name: "".to_string(),
                                                    original_name: ""
                                                        .to_string(),
                                                    date: "".to_string(),
                                                    has_all_names: false,
                                                },
                                                title: "".to_string(),
                                                description: "".to_string(),
                                                body: "".to_string(),
                                                permalink: "".to_string(),
                                                author: 0,
                                                author_name: "".to_string(),
                                                translator: q.data.userd.id,
                                                translator_name: ""
                                                    .to_string(),
                                                date: "".to_string(),
                                                date_trans: "".to_string(),
                                                draft: false,
                                                deleted: false,
                                            }
                                        ),
                                        featured_image: q.featured_image
                                            .clone(),
                                    },
                                    t: t,
                                    error: &Some(
                                        t_error(e, &q.data.lang.code),
                                    ),
                                    form: &Some(form),
                                };

                                HttpResponse::Ok().body(html.to_string())
                            }

                        }

                        Err(e2) => error_admin_route(e2, &user_req.lang_code),

                    },

                }

            },

            Err(_) => HttpResponse::Found()
                .header(
                    "Location",
                    ra_error_w_code(
                        &user_req.lang_code,
                        CSRF_TOKEN_IS_NOT_A_VALID_UUID,
                    ),
                )
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}
