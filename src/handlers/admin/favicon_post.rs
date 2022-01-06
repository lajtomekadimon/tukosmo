use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use actix_multipart::Multipart;
use postgres_types::{ToSql, FromSql};

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
    req: HttpRequest,
    id: Identity,
    payload: Multipart,
) -> impl Responder {

    match user_request(req, id) {

        Ok(user_req) => {if generate_favicon(payload).await {

            match query_db(
                ApiFavicon {
                    req: user_req.clone(),
                    //csrf_token: csrf_token_value,
                },
            ) {

                Ok(_row) => {

                    HttpResponse::Found()
                        .header(
                            "Location",
                            ra_favicon_success(&user_req.lang_code),
                        )
                        .finish()

                },

                Err(e) => match query_db(
                    AgiFavicon {
                        req: user_req.clone(),
                    },
                ) {

                    Ok(row) => {
                        let q: AgoFavicon = row.get(0);
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
