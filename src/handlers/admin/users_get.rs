use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::users::Users;


#[derive(Deserialize)]
pub struct GetParamData {
    rpp: Option<i64>,
    p: Option<i64>,
    success: Option<String>,
}


pub fn ra_users(
    lang_code: &str,
) -> String {
    "/{lang}/admin/users".replace("{lang}", lang_code)
}

pub fn ra_users_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/users?success=true"
        .replace("{lang}", lang_code)
}

pub fn ra_users_wu_rpp_p(
    lang_code: &str,
) -> String {
    "/{lang}/admin/users?rpp={rpp}&p={p}"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiUsers {
    pub req: types::AdminRequest,
    pub results_per_page: i64,
    pub page: i64,
}

impl QueryFunction for AgiUsers {
    fn query(&self) -> &str {
        "SELECT aha_g_users($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoUsers {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub users: Vec<types::UserDB>,
    pub results_per_page: i64,
    pub page: i64,
    pub total_results: i64,
    pub total_pages: i64,
}


pub async fn users_get(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    let results_per_page = (param.rpp).clone().unwrap_or(10);
    let current_page = (param.p).clone().unwrap_or(1);

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            AgiUsers {
                req: user_req.clone(),
                results_per_page: results_per_page,
                page: current_page,
            },
        ) {

            Ok(row) => {

                let q: AgoUsers = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Users {
                    title: &format!(
                        "{a} - {b}",
                        a = t.users,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: match param.success {
                        Some(_) => &true,
                        None => &false,
                    },
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

