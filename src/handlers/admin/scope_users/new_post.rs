use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::de::{Deserialize, Deserializer, Visitor, MapAccess};
use std::fmt;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::{
    user_request::user_request,
    scope_users::new_get::{
        AgiUsersNew,
        AgoUsersNew,
    },
    error_get::ra_error_w_code,
    users_get::ra_users_success,
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
use crate::templates::admin::scope_users::new::New;


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
pub struct ApiUsersNew {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub repeat_password: String,
    pub i18n_name_langs: Vec<i64>,
    pub i18n_names: Vec<String>,
}

impl QueryFunction for ApiUsersNew {
    fn query(&self) -> &str {
        "SELECT aha_p_users_new($1)"
    }
}


pub async fn new_post(
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
                    ApiUsersNew {
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

                        HttpResponse::Found()
                            .header(
                                "Location",
                                ra_users_success(&user_req.lang_code),
                            )
                            .finish()

                    },

                    Err(e) => match query_db(
                        AgiUsersNew {
                            req: user_req.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: AgoUsersNew = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = New {
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.new_user,
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                error: &Some(t_error(&e, &q.data.lang.code)),
                                form: &Some(form),
                            };

                            HttpResponse::Ok().body(html.to_string())

                        },

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

