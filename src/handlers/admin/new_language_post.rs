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
use crate::handlers::admin::new_language::{
    NewLanguageARequest,
    NewLanguageAResponse,
};
use crate::templates::admin::new_language::NewLanguage;


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
                let mut lang_code: String = "".to_string();
                let mut own_lang_name: String = "".to_string();
                let mut lang_ids: Vec<i64> = Vec::default();
                let mut lang_names: Vec<String> = Vec::default();
                let mut names_for_langs: Vec<String> = Vec::default();

                while let Some(key) = map.next_key()? {
                    match key {
                        "lang_code" => {
                            lang_code = map.next_value::<String>()?;
                        }
                        "own_lang_name" => {
                            own_lang_name = map.next_value::<String>()?;
                        }
                        "lang_id" => {
                            lang_ids.push(map.next_value::<i64>()?);
                        }
                        "lang_name" => {
                            lang_names.push(map.next_value::<String>()?);
                        }
                        "name_for_lang" => {
                            names_for_langs.push(map.next_value::<String>()?);
                        }
                        _ => unreachable!()
                    }
                }

                // TODO: unreachable!() if empty Vec or different lengths

                Ok(FormData {
                    lang_code: lang_code,
                    own_lang_name: own_lang_name,
                    lang_ids: lang_ids,
                    lang_names: lang_names,
                    names_for_langs: names_for_langs,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub struct FormData {
    pub lang_code: String,
    pub own_lang_name: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
    pub names_for_langs: Vec<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewLanguagePostARequest {
    pub req: types::AdminRequest,
    pub lang_code: String,
    pub own_lang_name: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
    pub names_for_langs: Vec<String>,
}

impl QueryFunction for NewLanguagePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_new_language_post($1)"
    }
}


pub async fn new_language_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {

            let lang_code = (form.lang_code).clone();
            let own_lang_name = (form.own_lang_name).clone();
            let lang_ids = (form.lang_ids).clone();
            let lang_names = (form.lang_names).clone();
            let names_for_langs = (form.names_for_langs).clone();

            match query_db(
                NewLanguagePostARequest {
                    req: user_req.clone(),
                    lang_code: lang_code,
                    own_lang_name: own_lang_name,
                    lang_ids: lang_ids,
                    lang_names: lang_names,
                    names_for_langs: names_for_langs,
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
                    NewLanguageARequest {
                        req: user_req.clone(),
                    },
                ) {

                    Ok(row) => {

                        let q: NewLanguageAResponse = row.get(0);

                        let html = NewLanguage {
                            title: &format!(
                                "{a} - {b}",
                                a = &t("Add language", &q.data.lang.code),
                                b = &t(
                                    "Tukosmo Admin Panel",
                                    &q.data.lang.code,
                                ),
                            ),
                            q: &q,
                            auto: &None,
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

