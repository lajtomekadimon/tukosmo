use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
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
use crate::templates::admin::scope_users::new::New;


pub fn ra_users_new(
    lang_code: &str,
) -> String {
    "/{lang}/admin/users/new".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiUsersNew {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiUsersNew {
    fn query(&self) -> &str {
        "SELECT aha_g_users_new($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoUsersNew {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn new_get(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
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
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

