use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::edit_user::EditUser;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditUserARequest {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for EditUserARequest {
    fn query(&self) -> &str {
        "SELECT awa_edit_user($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditUserAResponse {
    pub data: types::AdminDataDB,
    pub user_data: types::UserDB,
    pub i18n_names: Vec<types::NameDB>,
}


pub async fn edit_user(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let user_id = param.id;

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            EditUserARequest {
                req: user_req,
                id: user_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: EditUserAResponse = row.get(0);

                let html = EditUser {
                    title: &format!(
                        "{a} - {b}",
                        a = &t(
                            "Edit user: '{name}'",
                            &q.data.lang.code,
                        ).replace("{name}", &q.user_data.name),
                        b = &t("Tukosmo Admin Panel", &q.data.lang.code)
                    ),
                    q: &q,
                    error: &None,
                    form: &None,
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

