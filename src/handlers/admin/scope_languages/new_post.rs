use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::global::Config;
use crate::handlers::admin::{
    user_request::user_request,
    scope_languages::new_get::{
        AgiLanguagesNew,
        AgoLanguagesNew,
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
use crate::templates::admin::scope_languages::new::New;


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
                let mut lang_code: String = "".to_string();
                let mut own_lang_name: String = "".to_string();
                let mut lang_ids: Vec<i64> = Vec::default();
                let mut lang_names: Vec<String> = Vec::default();
                let mut names_for_langs: Vec<String> = Vec::default();
                let mut website_title_value: String = "".to_string();
                let mut website_subtitle_value: String = "".to_string();
                let mut copyright_owner_value: String = "".to_string();

                while let Some(key) = map.next_key()? {
                    match key {
                        "csrf_token" => {
                            csrf_token_value = map.next_value::<String>()?;
                        }
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
                        "website_title" => {
                            website_title_value = map.next_value::<String>()?;
                        }
                        "website_subtitle" => {
                            website_subtitle_value =
                                map.next_value::<String>()?;
                        }
                        "copyright_owner" => {
                            copyright_owner_value =
                                map.next_value::<String>()?;
                        }
                        _ => unreachable!()
                    }
                }

                // TODO: unreachable!() if empty Vec or different lengths

                Ok(FormData {
                    csrf_token: csrf_token_value,
                    lang_code: lang_code,
                    own_lang_name: own_lang_name,
                    lang_ids: lang_ids,
                    lang_names: lang_names,
                    names_for_langs: names_for_langs,
                    website_title: website_title_value,
                    website_subtitle: website_subtitle_value,
                    copyright_owner: copyright_owner_value,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub struct FormData {
    pub csrf_token: String,
    pub lang_code: String,
    pub own_lang_name: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
    pub names_for_langs: Vec<String>,
    pub website_title: String,
    pub website_subtitle: String,
    pub copyright_owner: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiLanguagesNew {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub lang_code: String,
    pub own_lang_name: String,
    pub lang_ids: Vec<i64>,
    pub lang_names: Vec<String>,
    pub names_for_langs: Vec<String>,
    pub website_title: String,
    pub website_subtitle: String,
    pub copyright_owner: String,
}

impl QueryFunction for ApiLanguagesNew {
    fn query(&self) -> &str {
        "SELECT aha_p_languages_new($1)"
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

                let lang_code = (form.lang_code).clone();
                let own_lang_name = (form.own_lang_name).clone();
                let lang_ids = (form.lang_ids).clone();
                let lang_names = (form.lang_names).clone();
                let names_for_langs = (form.names_for_langs).clone();
                let website_title = (form.website_title).clone();
                let website_subtitle = (form.website_subtitle).clone();
                let copyright_owner = (form.copyright_owner).clone();

                match query_db(
                    &config,
                    ApiLanguagesNew {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        lang_code: lang_code,
                        own_lang_name: own_lang_name,
                        lang_ids: lang_ids,
                        lang_names: lang_names,
                        names_for_langs: names_for_langs,
                        website_title: website_title,
                        website_subtitle: website_subtitle,
                        copyright_owner: copyright_owner,
                    },
                ) {

                    Ok(_row) => {

                        HttpResponse::Found()
                            .header(
                                "Location",
                                ra_languages_success(&user_req.lang_code),
                            )
                            .finish()
                    },

                    Err(e) => match query_db(
                        &config,
                        AgiLanguagesNew {
                            req: user_req.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: AgoLanguagesNew = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = New {
                                domain: &config.server.domain,
                                codename: &codename,
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.add_language,
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                auto: &None,
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

