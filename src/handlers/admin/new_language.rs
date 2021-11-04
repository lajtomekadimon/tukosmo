use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::config::global::SUPPORTED_LANGUAGES;
use crate::handlers::admin::user_request::user_request;
use crate::i18n::t::t;
use crate::i18n::error_admin_route::error_admin_route;
use crate::templates::admin::new_language::NewLanguage;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};


#[derive(Deserialize)]
pub struct GetParamData {
    auto: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewLanguageARequest {
    pub req: types::AdminRequest,
}

impl QueryFunction for NewLanguageARequest {
    fn query(&self) -> &str {
        "SELECT awa_new_language($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewLanguageAResponse {
    pub data: types::AdminDataDB,
    pub csrf_token: String,
}


pub async fn new_language(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            NewLanguageARequest {
                req: user_req.clone(),
            },
        ) {

            Ok(row) => {

                let q: NewLanguageAResponse = row.get(0);
                let t = &t(&q.data.lang.code);

                let maybe_auto_code = (param.auto).clone();

                let auto_code = match maybe_auto_code {
                    Some(the_auto_code) => if SUPPORTED_LANGUAGES.contains(
                        &the_auto_code.as_str()
                    ) { Some(the_auto_code) } else { None },
                    None => None,
                };

                let html = NewLanguage {
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

