use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::{
    global::Config,
    change_lang::change_lang,
};
use crate::server::new_server::Handle;
use crate::handlers::admin::{
    user_request::user_request,
    scope_languages::edit_get::{
        AgiLanguagesEdit,
        AgoLanguagesEdit,
    },
    error_get::ra_error_w_code,
    languages_get::ra_languages_success,
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
use crate::templates::admin::scope_languages::edit::Edit;


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
                let mut language_id: i64 = 0;
                let mut lang_code: String = "".to_string();
                let mut lang_ids: Vec<i64> = Vec::default();
                let mut lang_names: Vec<String> = Vec::default();

                while let Some(key) = map.next_key()? {
                    match key {
                        "csrf_token" => {
                            csrf_token_value = map.next_value::<String>()?;
                        }
                        "language_id" => {
                            language_id = map.next_value::<i64>()?;
                        }
                        "lang_code" => {
                            lang_code = map.next_value::<String>()?;
                        }
                        "lang_id" => {
                            lang_ids.push(map.next_value::<i64>()?);
                        }
                        "lang_name" => {
                            lang_names.push(map.next_value::<String>()?);
                        }
                        _ => unreachable!()
                    }
                }

                // TODO: unreachable!() if empty Vec or different lengths

                Ok(FormData {
                    csrf_token: csrf_token_value,
                    language_id: language_id,
                    lang_code: lang_code,
                    lang_ids: lang_ids,
                    lang_names: lang_names,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}


pub struct FormData {
    pub csrf_token: String,
    pub language_id: i64,
    pub lang_code: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiLanguagesEdit {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub language_id: i64,
    pub lang_code: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
}

impl QueryFunction for ApiLanguagesEdit {
    fn query(&self) -> &str {
        "SELECT aha_p_languages_edit($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApoLanguagesEdit {
    pub data: types::AdminDataDB,
    pub old_lang_code: String,
}


pub async fn edit_post(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
    handle: web::Data<Handle>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let language_id = (form.language_id).clone();
                let lang_code = (form.lang_code).clone();
                let lang_ids = (form.lang_ids).clone();
                let lang_names = (form.lang_names).clone();

                match query_db(
                    &config,
                    ApiLanguagesEdit {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        language_id: language_id,
                        lang_code: lang_code.clone(),
                        lang_ids: lang_ids,
                        lang_names: lang_names,
                    },
                ).await {

                    Ok(row) => {

                        let q: ApoLanguagesEdit = row.get(0);

                        if (&q.old_lang_code == &config.server.default_lang)
                            && (&lang_code != &q.old_lang_code)
                        {
                            change_lang(
                                &config,
                                &lang_code,
                            );
                            // TODO: Handle errors

                            // Restart server
                            let _ = handle.stop(true);
                        }

                        HttpResponse::Found()
                            .append_header((
                                "Location",
                                ra_languages_success(&lang_code),
                            ))
                            .finish()

                    },

                    Err(e) => match query_db(
                        &config,
                        AgiLanguagesEdit {
                            req: user_req.clone(),
                            lang: language_id,
                        },
                    ).await {

                        Ok(row) => {

                            let q: AgoLanguagesEdit = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = Edit {
                                domain: &config.server.domain,
                                codename: &codename,
                                config: &config,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.edit_language_w_name
                                        .replace("{name}", &q.lang.name),
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

