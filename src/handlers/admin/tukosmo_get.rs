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
use crate::templates::admin::tukosmo::Tukosmo;


#[derive(Deserialize)]
pub struct GetParamData {
    success: Option<String>,
}


pub fn ra_tukosmo(
    lang_code: &str,
) -> String {
    "/{lang}/admin/tukosmo".replace("{lang}", lang_code)
}

pub fn ra_tukosmo_success(
    lang_code: &str,
) -> String {
    "/{lang}/admin/tukosmo?success=true"
        .replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgiTukosmo {
    pub req: types::AdminRequest,
}

impl QueryFunction for AgiTukosmo {
    fn query(&self) -> &str {
        "SELECT aha_g_tukosmo($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AgoTukosmo {
    pub data: types::AdminDataDB,
    pub routes: Vec<types::RouteDB>,
    pub csrf_token: String,
}


pub async fn tukosmo_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    web::Query(param): web::Query<GetParamData>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            AgiTukosmo {
                req: user_req.clone(),
            },
        ).await {

            Ok(row) => {

                let q: AgoTukosmo = row.get(0);
                let t = &t(&q.data.lang.code);

                let html = Tukosmo {
                    domain: &config.server.domain,
                    codename: &codename,
                    config: &config,
                    title: &format!(
                        "{a} - {b}",
                        a = t.tukosmo,
                        b = t.tukosmo_admin_panel,
                    ),
                    q: &q,
                    t: t,
                    success: &param.success.is_some(),
                    error: &None,
                    form: &None,
                    new_version: &Some("0.2".to_string()),  // TODO: Update Tukosmo
                };

                HttpResponse::Ok()
                    .content_type("text/html; charset=UTF-8")
                    .body(html.to_string())

            },

            Err(e) => error_admin_route(&e, &user_req.lang_code),

        },

        Err(redirect_url) => redirect_url,

    }

}

