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
use crate::templates::admin::account::Account;


pub fn ra_account(
    lang_code: &str,
) -> String {
    "/{lang}/admin/account".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiAccount {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiAccount {
    fn query(&self) -> &str {
        "SELECT aha_g_account($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoAccount {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub user_data: types::UserDB,
    pub i18n_names: Vec<types::NameDB>,
}


pub async fn account_get(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            AgiAccount {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: AgoAccount = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Account {
                    title: &format!(
                        "{a} - {b}",
                        a = t.account,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: &false,
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

