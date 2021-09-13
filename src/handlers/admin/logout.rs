use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use uuid::Uuid;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::database::d_session::d_session;


pub async fn logout(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id.clone()) {

        Ok((lang_code, _user)) => {
            
            let login_route = "/{lang}/admin/login"
                .replace("{lang}", &lang_code);

            // Cookie has a session
            if let Some(session_uuid) = id.identity() {

                if let Ok(session_id) = Uuid::parse_str(
                    &session_uuid
                ) {

                    // Delete auth cookie
                    id.forget();

                    // Delete session from database
                    if let Ok(_) = d_session(session_id) {

                        // Redirect to login page
                        HttpResponse::Found()
                            .header("Location", login_route)
                            .finish()

                    } else {

                        // Redirect to dashboard
                        HttpResponse::Found()
                            .header(
                                "Location",
                                "/{lang}/admin"
                                    .replace("{lang}", &lang_code)
                                    // TODO: Couldn't logout
                            )
                            .finish()

                    }

                // TODO: "Session ID is not a valid UUID."
                } else {

                    // Delete cookie
                    id.forget();

                    // Redirect to login
                    HttpResponse::Found()
                        .header("Location", login_route)
                        .finish()

                }

            // No session
            // TODO: "You need to login first."
            } else {

                // Redirect to login
                HttpResponse::Found()
                    .header("Location", login_route)
                    .finish()

            }

        }

        Err(_e) => {

            // Redirect to login page
            HttpResponse::Found()
                .header(
                    "Location",
                    "/{lang}/admin/login"
                        .replace("{lang}", "en")  // TODO: default lang
                )
                .finish()

        }

    }
}
