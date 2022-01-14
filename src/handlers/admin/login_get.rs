use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::handlers::{
    website::user_request::user_request,
    admin::dashboard_get::ra_dashboard,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_website_route::error_website_route,
};
use crate::templates::admin::login::Login;


pub fn ra_login(
    lang_code: &str,
) -> String {
    "/{lang}/admin/login".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiLogin {
    pub req: types::WebsiteRequest,
}

impl QueryFunction for AgiLogin {
    fn query(&self) -> &str {
        "SELECT aha_g_login($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoLogin {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn login_get(
    config: web::Data<Config>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req.clone(), id.clone());

    match query_db(
        &config,
        AgiLogin {
            req: user_req.clone(),
        },
    ) {

        Ok(row) => {

            let q: AgoLogin = row.get(0);
            let t = &t(&q.data.lang.code);

            if let Some(_user) = q.data.userd {

                HttpResponse::Found()
                    .header(
                        "Location",
                        ra_dashboard(&q.data.lang.code),
                    )
                    .finish()

            } else {

                // Delete cookie
                id.forget();

                let html = Login {
                    domain: &config.server.domain,
                    title: &format!(
                        "{a} - {b}",
                        a = t.login_k_noun,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

        },

        // Website's error because the user is not logged in
        Err(e) => error_website_route(&e, &user_req.lang_code),

    }

}

