use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
    scope_posts::new_get::{
        AgiPostsNew,
        AgoPostsNew,
    },
    error_get::ra_error_w_code,
    posts_get::ra_posts_success,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID,
};
use crate::i18n::{
    t::t,
    t_error::t_error,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::scope_posts::new::New;


impl<'de> Deserialize<'de> for FormData {
    fn deserialize<D>(
        deserializer: D,
    ) -> Result<FormData, D::Error>
    where D: Deserializer<'de>
    {
        struct FieldVisitor;

        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = FormData;

            fn expecting(
                &self,
                formatter: &mut fmt::Formatter,
            ) -> fmt::Result {
                formatter.write_str("MESSAGE")  // TODO: expecting "a ___"
            }

            fn visit_map<V>(
                self,
                mut map: V
            ) -> Result<FormData, V::Error>
            where V: MapAccess<'de>
            {
                let mut csrf_token_value: String = "".to_string();
                let mut title_value: String = "".to_string();
                let mut featured_image_value: i64 = 0;
                let mut description_value: String = "".to_string();
                let mut body_value: String = "".to_string();
                let mut permalink_value: String = "".to_string();
                let mut tags: Vec<i64> = Vec::default();
                let mut draft_value: bool = false;

                while let Some(key) = map.next_key()? {
                    match key {
                        "csrf_token" => {
                            csrf_token_value = map.next_value::<String>()?;
                        }
                        "title" => {
                            title_value = map.next_value::<String>()?;
                        }
                        "featured_image" => {
                            featured_image_value = map.next_value::<i64>()?;
                        }
                        "description" => {
                            description_value = map.next_value::<String>()?;
                        }
                        "body" => {
                            body_value = map.next_value::<String>()?;
                        }
                        "permalink" => {
                            permalink_value = map.next_value::<String>()?;
                        }
                        "tag" => {
                            tags.push(map.next_value::<i64>()?);
                        }
                        "draft" => {
                            draft_value = true;
                        }
                        _ => unreachable!()
                    }
                }

                Ok(FormData {
                    csrf_token: csrf_token_value,
                    title: title_value,
                    featured_image: featured_image_value,
                    description: description_value,
                    body: body_value,
                    permalink: permalink_value,
                    tags: tags,
                    draft: draft_value,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub struct FormData {
    pub csrf_token: String,
    pub title: String,
    pub featured_image: i64,
    pub description: String,
    pub body: String,
    pub permalink: String,
    pub tags: Vec<i64>,
    pub draft: bool,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiPostsNew {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub title: String,
    pub description: String,
    pub body: String,
    pub permalink: String,
    pub draft: bool,
    pub featured_image: i64,
    pub tags: Vec<i64>,
}

impl QueryFunction for ApiPostsNew {
    fn query(&self) -> &str {
        "SELECT aha_p_posts_new($1)"
    }
}


pub async fn new_post(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let title_value = (form.title).clone();
                let featured_image_id = (form.featured_image).clone();
                let description_value = (form.description).clone();
                let body_value = (form.body).clone();
                let permalink_value = (form.permalink).clone();
                let is_draft = (form.draft).clone();
                let tags_added = (form.tags).clone();

                match query_db(
                    &config,
                    ApiPostsNew {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        title: title_value,
                        description: description_value,
                        body: body_value,
                        permalink: permalink_value,
                        draft: is_draft,
                        featured_image: featured_image_id,
                        tags: tags_added.clone(),
                    },
                ).await {

                    Ok(_) => {

                        HttpResponse::Found()
                            .append_header((
                                "Location",
                                ra_posts_success(&user_req.lang_code),
                            ))
                            .finish()

                    },

                    Err(e) => match query_db(
                        &config,
                        AgiPostsNew {
                            req: user_req.clone(),
                            featured_image: if featured_image_id == 0 {
                                None
                            } else { Some(featured_image_id) },
                            tags_added: tags_added,
                        },
                    ).await {

                        Ok(row) => {

                            let q: AgoPostsNew = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = New {
                                domain: &config.server.domain,
                                codename: &codename,
                                config: &config,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.new_post,
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                error: &Some(t_error(&e, &q.data.lang.code)),
                                form: &Some(form),
                            };

                            HttpResponse::Ok()
                                .content_type("text/html; charset=UTF-8")
                                .body(html.to_string())

                        }

                        Err(e2) => error_admin_route(&e2, &user_req.lang_code),

                    },

                }

            },

            Err(_) => HttpResponse::Found()
                .append_header((
                    "Location",
                    ra_error_w_code(
                        &user_req.lang_code,
                        CSRF_TOKEN_IS_NOT_A_VALID_UUID,
                    ),
                ))
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}

