use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
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
use crate::templates::admin::scope_tags::edit::Edit;


#[derive(Deserialize)]
pub struct GetParamData {
    id: i64,
}


pub fn ra_tags_edit_w_id(
    lang_code: &str,
    id: &i64,
) -> String {
    "/{lang}/admin/tags/edit?id={id}"
        .replace("{lang}", lang_code)
        .replace("{id}", &id.to_string())
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiTagsEdit {
    pub req: types::AdminRequest,
    pub tag: i64,
}

impl QueryFunction for AgiTagsEdit {
    fn query(&self) -> &str {
        "SELECT aha_g_tags_edit($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoTagsEdit {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
    pub tag: types::TagDB,
    pub tag_translations: Vec<types::TagDB>,
}


pub async fn edit_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiTagsEdit {
                req: user_req.clone(),
                tag: param.id,
            },
        ) {

            Ok(row) => {

                let q: AgoTagsEdit = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Edit {
                    domain: &config.server.domain,
                    codename: &codename,
                    title: &format!(
                        "{a} - {b}",
                        a = t.edit_tag_w_name
                            .replace("{name}", &q.tag.name),
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    error: &None,
                    form: &None,
                };

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            }

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

