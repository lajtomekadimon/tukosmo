use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::database::error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::new_user::{
    NewUserARequest,
    NewUserAResponse,
};
use crate::templates::admin::new_user::NewUser;


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
                let mut name_value: String = "".to_string();
                let mut email_value: String = "".to_string();
                let mut password_value: String = "".to_string();
                let mut repeat_password_value: String = "".to_string();
                let mut i18n_name_langs: Vec<i64> = Vec::default();
                let mut i18n_names: Vec<String> = Vec::default();

                while let Some(key) = map.next_key()? {
                    match key {
                        "csrf_token" => {
                            csrf_token_value = map.next_value::<String>()?;
                        }
                        "name" => {
                            name_value = map.next_value::<String>()?;
                        }
                        "email" => {
                            email_value = map.next_value::<String>()?;
                        }
                        "password" => {
                            password_value = map.next_value::<String>()?;
                        }
                        "repeat_password" => {
                            repeat_password_value =
                                map.next_value::<String>()?;
                        }
                        "i18n_name_lang" => {
                            i18n_name_langs.push(map.next_value::<i64>()?);
                        }
                        "i18n_name" => {
                            i18n_names.push(map.next_value::<String>()?);
                        }
                        _ => unreachable!()
                    }
                }

                // TODO: unreachable!() if empty Vec or different lengths

                Ok(FormData {
                    csrf_token: csrf_token_value,
                    name: name_value,
                    email: email_value,
                    password: password_value,
                    repeat_password: repeat_password_value,
                    i18n_name_langs: i18n_name_langs,
                    i18n_names: i18n_names,
                })
            }
        }

        deserializer.deserialize_identifier(FieldVisitor)
    }
}

pub struct FormData {
    pub csrf_token: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub repeat_password: String,
    pub i18n_name_langs: Vec<i64>,
    pub i18n_names: Vec<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewUserPostARequest {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub repeat_password: String,
    pub i18n_name_langs: Vec<i64>,
    pub i18n_names: Vec<String>,
}

impl QueryFunction for NewUserPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_new_user_post($1)"
    }
}


pub async fn new_user_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let name_value = (form.name).clone();
                let email_value = (form.email).clone();
                let password_value = (form.password).clone();
                let repeat_password_value = (form.repeat_password).clone();
                let i18n_name_langs = (form.i18n_name_langs).clone();
                let i18n_names = (form.i18n_names).clone();

                match query_db(
                    NewUserPostARequest {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        name: name_value,
                        email: email_value,
                        password: password_value,
                        repeat_password: repeat_password_value,
                        i18n_name_langs: i18n_name_langs,
                        i18n_names: i18n_names,
                    },
                ) {

                    Ok(_) => {

                        let redirect_route = "/{lang}/admin/users?success=yes"
                            .replace("{lang}", &user_req.lang_code);

                        HttpResponse::Found()
                            .header("Location", redirect_route)
                            .finish()

                    },

                    Err(e) => match query_db(
                        NewUserARequest {
                            req: user_req.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: NewUserAResponse = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = NewUser {
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.new_user,
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                error: &Some(t_error(e, &q.data.lang.code)),
                                form: &Some(form),
                            };

                            HttpResponse::Ok().body(html.to_string())

                        },

                        Err(e2) => error_admin_route(e2, &user_req.lang_code),

                    },

                }

            },

            Err(_) => HttpResponse::Found()
                .header("Location", "/{lang}/admin/error?code={code}"
                    .replace("{lang}", &user_req.lang_code)
                    .replace("{code}", CSRF_TOKEN_IS_NOT_A_VALID_UUID)
                )
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}

