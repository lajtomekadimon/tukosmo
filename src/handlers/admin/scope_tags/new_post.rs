use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
    scope_tags::new_get::{
        AgiTagsNew,
        AgoTagsNew,
    },
    error_get::ra_error_w_code,
    tags_get::ra_tags_success,
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
use crate::templates::admin::scope_tags::new::New;


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
                let mut lang_ids: Vec<i64> = Vec::default();
                let mut tag_names: Vec<String> = Vec::default();
                let mut tag_permalinks: Vec<String> = Vec::default();

                while let Some(key) = map.next_key()? {
                    match key {
                        "csrf_token" => {
                            csrf_token_value = map.next_value::<String>()?;
                        }
                        "lang_id" => {
                            lang_ids.push(map.next_value::<i64>()?);
                        }
                        "tag_name" => {
                            tag_names.push(map.next_value::<String>()?);
                        }
                        "tag_permalink" => {
                            tag_permalinks.push(map.next_value::<String>()?);
                        }
                        _ => unreachable!()
                    }
                }

                // TODO: unreachable!() if empty Vec or different lengths

                Ok(FormData {
                    csrf_token: csrf_token_value,
                    lang_ids: lang_ids,
                    tag_names: tag_names,
                    tag_permalinks: tag_permalinks,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub struct FormData {
    pub csrf_token: String,
    pub lang_ids: Vec<i64>,
    pub tag_names: Vec<String>,
    pub tag_permalinks: Vec<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiTagsNew {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub lang_ids: Vec<i64>,
    pub tag_names: Vec<String>,
    pub tag_permalinks: Vec<String>,
}

impl QueryFunction for ApiTagsNew {
    fn query(&self) -> &str {
        "SELECT aha_p_tags_new($1)"
    }
}


pub async fn new_post(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let lang_ids = (form.lang_ids).clone();
                let tag_names = (form.tag_names).clone();
                let tag_permalinks = (form.tag_permalinks).clone();

                match query_db(
                    &config,
                    ApiTagsNew {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        lang_ids: lang_ids,
                        tag_names: tag_names,
                        tag_permalinks: tag_permalinks,
                    },
                ).await {

                    Ok(_row) => {

                        HttpResponse::Found()
                            .append_header((
                                "Location",
                                ra_tags_success(&user_req.lang_code),
                            ))
                            .finish()
                    },

                    Err(e) => match query_db(
                        &config,
                        AgiTagsNew {
                            req: user_req.clone(),
                        },
                    ).await {

                        Ok(row) => {

                            let q: AgoTagsNew = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = New {
                                domain: &config.server.domain,
                                codename: &codename,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.new_tag,
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

