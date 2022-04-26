use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::config::{
    global::Config,
    change_modules::change_modules,
};
use crate::server::new_server::Handle;
use crate::handlers::admin::{
    user_request::user_request,
    tukosmo_get::{
        AgiTukosmo,
        AgoTukosmo,
        ra_tukosmo_success,
    },
    error_get::ra_error_w_code,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID,
};
use crate::i18n::{
    t::t,
    t_error::t_error,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::tukosmo::Tukosmo;


#[derive(Deserialize)]
pub struct FormData {
    pub csrf_token: String,
    pub module_blog: Option<String>,
    pub module_gallery: Option<String>,
    pub module_faq: Option<String>,
    pub module_payments: Option<String>,
    pub module_subscriptions: Option<String>,
    pub module_shop: Option<String>,
}


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiTukosmo {
    pub req: types::AdminRequest,
    pub csrf_token: Uuid,
}

impl QueryFunction for ApiTukosmo {
    fn query(&self) -> &str {
        "SELECT aha_p_tukosmo($1)"
    }
}


pub async fn tukosmo_post(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    form: web::Form<FormData>,
    handle: web::Data<Handle>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match Uuid::parse_str(&(form.csrf_token).clone()) {

            Ok(csrf_token_value) => {

                let module_blog = (form.module_blog).clone().is_some();
                let module_gallery = (form.module_gallery).clone().is_some();
                let module_faq = (form.module_faq).clone().is_some();
                let module_payments = (form.module_payments).clone().is_some();
                let module_subscriptions =
                    (form.module_subscriptions).clone().is_some();
                let module_shop = (form.module_shop).clone().is_some();

                match query_db(
                    &config,
                    ApiTukosmo {
                        req: user_req.clone(),
                        csrf_token: csrf_token_value,
                    },
                ).await {

                    Ok(_row) => {

                        // TODO: Do only when the form changes
                        change_modules(
                            &config,
                            module_blog,
                            module_gallery,
                            module_faq,
                            module_payments,
                            module_subscriptions,
                            module_shop,
                        );
                        // TODO: Handle errors

                        // Restart server
                        let _ = handle.stop(true);

                        HttpResponse::Found()
                            .append_header((
                                "Location",
                                ra_tukosmo_success(&user_req.lang_code),
                            ))
                            .finish()

                    },

                    Err(e) => match query_db(
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
                                success: &false,
                                error: &Some(
                                    t_error(&e, &q.data.lang.code),
                                ),
                                form: &Some(form),
                                new_version: &None,  // TODO: Update Tukosmo
                            };

                            HttpResponse::Ok()
                                .content_type("text/html; charset=UTF-8")
                                .body(html.to_string())

                        }

                        Err(e2) => error_admin_route(&e2, &user_req.lang_code),

                    },

                }

            },

            Err(_) => HttpResponse::Found()
                .append_header((
                    "Location",
                    ra_error_w_code(
                        &user_req.lang_code,
                        CSRF_TOKEN_IS_NOT_A_VALID_UUID,
                    ),
                ))
                .finish(),

        },

        Err(redirect_url) => redirect_url,

    }

}
