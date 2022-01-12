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
use crate::templates::admin::scope_languages::edit::Edit;


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


pub fn ra_languages_edit_w_id(
    lang_code: &str,
    id: &i64,
) -> String {
    "/{lang}/admin/languages/edit?id={id}"
        .replace("{lang}", lang_code)
        .replace("{id}", &id.to_string())
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiLanguagesEdit {
    pub req: types::AdminRequest,
    pub lang: i64,
}

impl QueryFunction for AgiLanguagesEdit {
    fn query(&self) -> &str {
        "SELECT aha_g_languages_edit($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoLanguagesEdit {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub lang: types::LanguageDB,
    pub names: Vec<types::NameDB>,
}


pub async fn edit_get(
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            AgiLanguagesEdit {
                req: user_req.clone(),
                lang: param.id,
            },
        ) {

            Ok(row) => {

                let q: AgoLanguagesEdit = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Edit {
                    title: &format!(
                        "{a} - {b}",
                        a = t.edit_language_w_name
                            .replace("{name}", &q.lang.name),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok().body(html.to_string())

            }

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

