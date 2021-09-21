use actix_web::{HttpRequest, HttpResponse};
use actix_identity::Identity;
use uuid::Uuid;

use crate::database::awa_admin_handler::awa_admin_handler;
use crate::database::types::DataDB;


pub fn admin_handler(
    req: HttpRequest,
    id: Identity,
) -> Result<DataDB, HttpResponse> {

    let lang_code: String = req.match_info()
        .get("lang").unwrap().parse().unwrap();

    let login_route = "/{lang}/admin/login"
        .replace("{lang}", &lang_code);

    // Cookie has a session
    if let Some(session_uuid) = id.identity() {

        // Session ID is a valid UUID
        if let Ok(session_id) = Uuid::parse_str(
            &session_uuid
        ) {

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

                // Redirect to login page
                Err(
                    HttpResponse::Found()
                        .header("Location", login_route)
                        .finish()
                )

            }

        // TODO: "Session ID is not a valid UUID."
        } else {

            // Delete cookie
            id.forget();

            // Redirect to login page
            Err(
                HttpResponse::Found()
                    .header("Location", login_route)
                    .finish()
            )

        }

    // No session
    // TODO: "You need to login first."
    } else {

        // Redirect to login page
        Err(
            HttpResponse::Found()
                .header("Location", login_route)
                .finish()
        )

    }

}

