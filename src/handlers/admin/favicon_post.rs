use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use actix_multipart::Multipart;
use postgres_types::{ToSql, FromSql};

use crate::config::global::Config;
use crate::server::new_server::Handle;
use crate::handlers::admin::{
    user_request::user_request,
    favicon_get::{
        AgiFavicon,
        AgoFavicon,
        ra_favicon_success,
    },
};
use crate::files::generate_favicon::generate_favicon;
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
    //error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
};
use crate::i18n::{
    t::t,
    t_error::t_error,
    error_admin_route::error_admin_route,
};
use crate::templates::admin::favicon::Favicon;


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ApiFavicon {
    pub req: types::AdminRequest,
    //pub csrf_token: Uuid,
}

impl QueryFunction for ApiFavicon {
    fn query(&self) -> &str {
        "SELECT aha_p_favicon($1)"
    }
}


pub async fn favicon_post(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
    payload: Multipart,
    handle: web::Data<Handle>,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => match query_db(
            &config,
            ApiFavicon {
                req: user_req.clone(),
                //csrf_token: csrf_token_value,
            },
        ).await {

            Ok(_row) => {if generate_favicon(payload).await {

                // Restart server
                let _ = handle.stop(true);

                HttpResponse::Found()
                    .append_header((
                        "Location",
                        ra_favicon_success(&user_req.lang_code),
                    ))
                    .finish()

            } else {

                // TODO
                HttpResponse::Ok().body("Error!!!")

            }},

            Err(e) => match query_db(
                &config,
                AgiFavicon {
                    req: user_req.clone(),
                },
            ).await {

                Ok(row) => {
                    let q: AgoFavicon = row.get(0);
                    let t = &t(&q.data.lang.code);

                    let html = Favicon {
                        domain: &config.server.domain,
                        codename: &codename,
                        config: &config,
                        title: &format!(
                            "{a} - {b}",
                            a = t.favicon,
                            b = t.tukosmo_admin_panel,
                        ),
                        q: &q,
                        t: t,
                        error: &Some(t_error(&e, &q.data.lang.code)),
                        success: &false,
                    };

                    HttpResponse::Ok()
                        .content_type("text/html; charset=UTF-8")
                        .body(html.to_string())

                },

                Err(e2) => error_admin_route(&e2, &user_req.lang_code),

            },

        },

        Err(redirect_url) => redirect_url,

    }

}
