use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::edit_language::{
    EditLanguageARequest,
    EditLanguageAResponse,
};
use crate::templates::admin::edit_language::EditLanguage;


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
                let mut language_id: i64 = 0;
                let mut lang_code: String = "".to_string();
                let mut lang_ids: Vec<i64> = Vec::default();
                let mut lang_names: Vec<String> = Vec::default();

                while let Some(key) = map.next_key()? {
                    match key {
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
    pub language_id: i64,
    pub lang_code: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguagePostARequest {
    pub req: types::AdminRequest,
    pub language_id: i64,
    pub lang_code: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
}

impl QueryFunction for EditLanguagePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_language_post($1)"
    }
}


pub async fn edit_language_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {

            let language_id = (form.language_id).clone();
            let lang_code = (form.lang_code).clone();
            let lang_ids = (form.lang_ids).clone();
            let lang_names = (form.lang_names).clone();

            match query_db(
                EditLanguagePostARequest {
                    req: user_req.clone(),
                    language_id: language_id,
                    lang_code: lang_code,
                    lang_ids: lang_ids,
                    lang_names: lang_names,
                },
            ) {

                Ok(_row) => {

                    let redirect_route = "/{lang}/admin/languages?success=yes"
                        .replace("{lang}", &user_req.lang_code);

                    HttpResponse::Found()
                        .header("Location", redirect_route)
                        .finish()
                },

                Err(e) => match query_db(
                    EditLanguageARequest {
                        req: user_req.clone(),
                        lang: language_id,
                    },
                ) {

                    Ok(row) => {

                        let q: EditLanguageAResponse = row.get(0);
                        let t = &t(&q.data.lang.code);

                        let html = EditLanguage {
                            title: &format!(
                                "{a} - {b}",
                                a = t.edit_language_w_name
                                    .replace("{name}", &q.lang.name),
                                b = t.tukosmo_admin_panel,
                            ),
                            q: &q,
                            t: t,
                            error: &Some(t_error(e, &q.data.lang.code)),
                            form: &Some(form),
                        };

                        HttpResponse::Ok().body(html.to_string())

                    }

                    Err(e2) => error_admin_route(e2, &user_req.lang_code),

                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}

