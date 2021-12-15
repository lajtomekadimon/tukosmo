use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};

use crate::handlers::website::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::admin::login::Login;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::handlers::admin::dashboard::ra_dashboard;


pub fn ra_login(
    lang_code: &str,
) -> String {
    "/{lang}/admin/login".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginARequest {
    pub req: types::WebsiteRequest,
}

impl QueryFunction for LoginARequest {
    fn query(&self) -> &str {
        "SELECT awa_login($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginAResponse {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
}


pub async fn login(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req.clone(), id.clone());

    match query_db(
        LoginARequest {
            req: user_req.clone(),
        },
    ) {

        Ok(row) => {

            let q: LoginAResponse = row.get(0);
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

        Err(e) => {
            println!("{}", e);  // for debugging

            let error_route = "/{lang}/error"
                .replace("{lang}", &user_req.lang_code);

            HttpResponse::Found()
                .header("Location", error_route)
                .finish()
        },

    }

}

