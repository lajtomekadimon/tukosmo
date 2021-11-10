use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
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
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
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
                req: user_req.clone(),
                id: user_id.clone(),
            },
        ) {

            Ok(row) => {

                let q: EditUserAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = EditUser {
                    title: &format!(
                        "{a} - {b}",
                        a = t.edit_user_w_name
                            .replace("{name}", &q.user_data.name),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

