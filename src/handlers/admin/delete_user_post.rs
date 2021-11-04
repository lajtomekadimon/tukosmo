use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::database::error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::delete_user::{
    DeleteUserARequest,
    DeleteUserAResponse,
};
use crate::templates::admin::delete_user::DeleteUser;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteUserPostARequest {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
    pub id: i64,
}

impl QueryFunction for DeleteUserPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_delete_user_post($1)"
    }
}


pub async fn delete_user_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let user_id = (form.id).clone();

                match query_db(
                    DeleteUserPostARequest {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                        id: user_id.clone(),
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
                        DeleteUserARequest {
                            req: user_req.clone(),
                            id: user_id.clone(),
                        },
                    ) {

                        Ok(row) => {

                            let q: DeleteUserAResponse = row.get(0);
                            let t = &t(&q.data.lang.code);

                            let html = DeleteUser {
                                title: &format!(
                                    "{a} - {b}",
                                    a = t.delete_user_w_name
                                        .replace("{name}", &q.user_data.name),
                                    b = t.tukosmo_admin_panel,
                                ),
                                q: &q,
                                t: t,
                                error: &Some(t_error(e, &q.data.lang.code)),
                            };

                            HttpResponse::Ok().body(html.to_string())

                        }

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
