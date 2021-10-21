use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::handlers::admin::new_user::{
    NewUserARequest,
    NewUserAResponse,
};
use crate::templates::admin::new_user::NewUser;


#[derive(Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
    pub password: String,
    pub repeat_password: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewUserPostARequest {
    pub req: types::AdminRequest,
    pub name: String,
    pub email: String,
    pub password: String,
    pub repeat_password: String,
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

        Ok(user_req) => {

            let name_value = (form.name).clone();
            let email_value = (form.email).clone();
            let password_value = (form.password).clone();
            let repeat_password_value = (form.repeat_password).clone();

            match query_db(
                NewUserPostARequest {
                    req: user_req.clone(),
                    name: name_value,
                    email: email_value,
                    password: password_value,
                    repeat_password: repeat_password_value,
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
                        req: user_req,
                    },
                ) {

                    Ok(row) => {

                        let q: NewUserAResponse = row.get(0);

                        let html = NewUser {
                            title: &format!(
                                "{a} - {b}",
                                a = &t("New user", &q.data.lang.code),
                                b = &t(
                                    "Tukosmo Admin Panel",
                                    &q.data.lang.code,
                                ),
                            ),
                            q: &q,
                            error: &Some(t_error(e, &q.data.lang.code)),
                            form: &Some(form),
                        };

                        HttpResponse::Ok().body(html.to_string())

                    },

                    Err(e2) => {
                        println!("{}", e2);
                        HttpResponse::Found()
                            .header("Location", "/")  // TODO
                            .finish()
                    },

                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}

