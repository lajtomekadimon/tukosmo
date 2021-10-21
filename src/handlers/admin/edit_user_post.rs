use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::handlers::admin::edit_user::{
    EditUserARequest,
    EditUserAResponse,
};
use crate::templates::admin::edit_user::EditUser;


#[derive(Deserialize)]
pub struct FormData {
    pub id: i64,
    pub email: String,
    pub name: String,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditUserPostARequest {
    pub req: types::AdminRequest,
    pub id: i64,
    pub email: String,
    pub name: String,
}

impl QueryFunction for EditUserPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_user_post($1)"
    }
}


pub async fn edit_user_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {

            let user_id = (form.id).clone();
            let email_value = (form.email).clone();
            let name_value = (form.name).clone();

            match query_db(
                EditUserPostARequest {
                    req: user_req.clone(),
                    id: user_id,
                    email: email_value,
                    name: name_value,
                },
            ) {

                Ok(_row) => {

                    let redirect_route = "/{lang}/admin/users?success=yes"
                        .replace("{lang}", &user_req.lang_code);

                    HttpResponse::Found()
                        .header("Location", redirect_route)
                        .finish()

                },

                Err(e) => match query_db(
                    EditUserARequest {
                        req: user_req,
                        id: user_id,
                    },
                ) {

                    Ok(row) => {

                        let q: EditUserAResponse = row.get(0);

                        let html = EditUser {
                            title: &format!(
                                "{a} - {b}",
                                a = &t(
                                    "Edit user: '{name}'",
                                    &q.data.lang.code
                                ).replace("{name}", &q.user_data.name),
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

                    }

                    Err(e) => {
                        println!("{}", e);
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
