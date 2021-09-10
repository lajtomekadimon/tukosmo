use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::handlers::admin::admin_handler::admin_handler;


pub async fn logout(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id.clone()) {

        Ok((lang_code, _user)) => {

            // Delete auth cookie
            id.forget();

            // TODO: Delete session from database!

            // Redirect to login page
            HttpResponse::Found()
                .header(
                    "Location",
                    "/{lang}/admin/login".replace("{lang}", &lang_code)
                )
                .finish()

        }

        Err(_e) => {

            // Redirect to login page
            HttpResponse::Found()
                .header(
                    "Location",
                    "/"  // TODO
                )
                .finish()
        }

    }
}
