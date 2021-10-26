use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::delete_user::DeleteUser;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteUserARequest {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for DeleteUserARequest {
    fn query(&self) -> &str {
        "SELECT awa_delete_user($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteUserAResponse {
    pub data: types::AdminDataDB,
    pub user_data: types::UserDB,
}


pub async fn delete_user(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let user_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            DeleteUserARequest {
                req: user_req,
                id: user_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: DeleteUserAResponse = row.get(0);

                let html = DeleteUser {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Delete user: {name}",
                            &q.data.lang.code
                        ).replace("{name}", &q.user_data.name),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
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

        Err(redirect_url) => redirect_url,

    }

}

