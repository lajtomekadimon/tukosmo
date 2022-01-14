use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::pages::Pages;


pub fn ra_pages(
    lang_code: &str,
) -> String {
    "/{lang}/admin/pages".replace("{lang}", lang_code)
}

/*
pub fn ra_pages_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/pages?success=true"
        .replace("{lang}", lang_code)
}
*/

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiPages {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiPages {
    fn query(&self) -> &str {
        "SELECT aha_g_pages($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoPages {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn pages_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiPages {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: AgoPages = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Pages {
                    domain: &config.server.domain,
                    title: &format!(
                        "{a} - {b}",
                        a = t.pages,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                };

                HttpResponse::Ok().body(html.to_string())

            },

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

