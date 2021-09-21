use actix_web::{HttpRequest, HttpResponse};
use actix_identity::Identity;
use uuid::Uuid;

use crate::database::awa_admin_handler::awa_admin_handler;
use crate::database::types::DataDB;


pub fn admin_handler(
    req: HttpRequest,
    id: Identity,
) -> Result<DataDB, HttpResponse> {

    // Cookie has a session
    if let Some(session_uuid) = id.identity() {

        // Session ID is a valid UUID
        if let Ok(session_id) = Uuid::parse_str(
            &session_uuid
        ) {

            let lang_code: String = req.match_info()
                .get("lang").unwrap().parse().unwrap();

            // Session is active
            if let Some(data) = awa_admin_handler(
                session_id,
                lang_code,
            ) {

                Ok(data)

            // Session has expired
            // TODO: "Your session has expired."
            // TODO: This happens even if the user is correct!
            //       (when lang_code is not correct)
            } else {

                // Delete cookie
                id.forget();

                // TODO: Redirect to login
                Err(HttpResponse::Ok().body("Error 401: Unauthorized"))

            }

        // TODO: "Session ID is not a valid UUID."
        } else {

            // Delete cookie
            id.forget();

            // TODO: Redirect to login
            Err(HttpResponse::Ok().body("Error 401: Unauthorized"))

        }

    // No session
    // TODO: "You need to login first."
    } else {

        // TODO: Redirect to login
        Err(HttpResponse::Ok().body("Error 401: Unauthorized"))

    }

}

