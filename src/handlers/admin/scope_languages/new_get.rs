use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::SUPPORTED_LANGUAGES;
use crate::handlers::admin::user_request::user_request;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::scope_languages::new::New;


#[derive(Deserialize)]
pub struct GetParamData {
    auto: Option<String>,
}


pub fn ra_languages_new(
    lang_code: &str,
) -> String {
    "/{lang}/admin/languages/new".replace("{lang}", lang_code)
}

pub fn ra_languages_new_w_auto(
    lang_code: &str,
    auto: &str,
) -> String {
    "/{lang}/admin/languages/new?auto={auto}"
        .replace("{lang}", lang_code)
        .replace("{auto}", auto)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiLanguagesNew {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiLanguagesNew {
    fn query(&self) -> &str {
        "SELECT aha_g_languages_new($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoLanguagesNew {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn new_get(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            AgiLanguagesNew {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: AgoLanguagesNew = row.get(0);
                let t = &t(&q.data.lang.code);

                let maybe_auto_code = (param.auto).clone();

                let auto_code = match maybe_auto_code {
                    Some(the_auto_code) => if SUPPORTED_LANGUAGES.contains(
                        &the_auto_code.as_str()
                    ) { Some(the_auto_code) } else { None },
                    None => None,
                };

                let html = New {
                    title: &format!(
                        "{a} - {b}",
                        a = t.add_language,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    auto: &auto_code,
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

