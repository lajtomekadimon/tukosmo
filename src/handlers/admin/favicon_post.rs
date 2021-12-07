use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use actix_multipart::Multipart;
use postgres_types::{ToSql, FromSql};

use crate::handlers::admin::user_request::user_request;
use crate::database::types;
use crate::database::query_db::{QueryFunction, query_db};
//use crate::database::error_codes::CSRF_TOKEN_IS_NOT_A_VALID_UUID;
use crate::i18n::t::t;
use crate::i18n::t_error::t_error;
use crate::i18n::error_admin_route::error_admin_route;
use crate::handlers::admin::favicon::{
    FaviconARequest,
    FaviconAResponse,
};
use crate::templates::admin::favicon::Favicon;
use crate::files::generate_favicon::generate_favicon;


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct FaviconPostARequest {
    pub req: types::AdminRequest,
    //pub csrf_token: Uuid,
}

impl QueryFunction for FaviconPostARequest {
    fn query(&self) -> &str {
        "SELECT awa_favicon_post($1)"
    }
}


pub async fn favicon_post(
    req: HttpRequest,
    id: Identity,
    payload: Multipart,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {if generate_favicon(payload).await {

            match query_db(
                FaviconPostARequest {
                    req: user_req.clone(),
                    //csrf_token: csrf_token_value,
                },
            ) {

                Ok(_row) => {

                    let redirect_route = "/{lang}/admin/favicon?success=true"
                        .replace("{lang}", &user_req.lang_code);

                    HttpResponse::Found()
                        .header("Location", redirect_route)
                        .finish()

                },

                Err(e) => match query_db(
                    FaviconARequest {
                        req: user_req.clone(),
                    },
                ) {

                    Ok(row) => {
                        let q: FaviconAResponse = row.get(0);
                        let t = &t(&q.data.lang.code);

                        let html = Favicon {
                            title: &format!(
                                "{a} - {b}",
                                a = t.favicon,
                                b = t.tukosmo_admin_panel,
                            ),
                            q: &q,
                            t: t,
                            error: &Some(t_error(e, &q.data.lang.code)),
                            success: &false,
                        };

                        HttpResponse::Ok().body(html.to_string())

                    },

                    Err(e2) => error_admin_route(e2, &user_req.lang_code),

                },

            }

        } else {

            // TODO
            HttpResponse::Ok().body("Error!!!")

        }},

        Err(redirect_url) => redirect_url,

    }

}
