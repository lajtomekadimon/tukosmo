use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::delete_language::{
    DeleteLanguageARequest,
    DeleteLanguageAResponse,
};
use crate::templates::admin::delete_language::DeleteLanguage;


#[derive(Deserialize)]
pub struct FormData {
    pub id: i64,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DeleteLanguagePostARequest {
    pub req: types::AdminRequest,
    pub id: i64,
}

impl QueryFunction for DeleteLanguagePostARequest {
    fn query(&self) -> &str {
        "SELECT awa_delete_language_post($1)"
    }
}


pub async fn delete_language_post(
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {

            let lang_id = (form.id).clone();

            match query_db(
                DeleteLanguagePostARequest {
                    req: user_req.clone(),
                    id: lang_id.clone(),
                },
            ) {

                Ok(_row) => {

                    let redirect_route = "/{lang}/admin/languages?success=yes"
                        .replace("{lang}", &user_req.lang_code);

                    HttpResponse::Found()
                        .header("Location", redirect_route)
                        .finish()

                },

                Err(e) => match query_db(
                    DeleteLanguageARequest {
                        req: user_req.clone(),
                        id: lang_id.clone(),
                    },
                ) {

                    Ok(row) => {

                        let q: DeleteLanguageAResponse = row.get(0);

                        let html = DeleteLanguage {
                            title: &format!(
                                "{a} - {b}",
                                a = &t(
                                    "Delete language: {name}",
                                    &q.data.lang.code
                                ).replace("{name}", &q.lang.name),
                                b = &t(
                                    "Tukosmo Admin Panel",
                                    &q.data.lang.code,
                                ),
                            ),
                            q: &q,
                            error: &Some(t_error(e, &q.data.lang.code)),
                        };

                        HttpResponse::Ok().body(html.to_string())

                    }

                    Err(e2) => error_admin_route(e2, &user_req.lang_code),

                },

            }

        },

        Err(redirect_url) => redirect_url,

    }

}
