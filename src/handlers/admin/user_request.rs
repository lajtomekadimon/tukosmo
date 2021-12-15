use actix_web::{HttpRequest, HttpResponse};
use actix_identity::Identity;
use uuid::Uuid;

use crate::database::types::AdminRequest;
use crate::handlers::admin::login::ra_login;


pub fn user_request(
    req: HttpRequest,
    id: Identity,
) -> Result<AdminRequest, HttpResponse> {

    let lang_code: String = req.match_info()
        .get("lang").unwrap().parse().unwrap();

    // Cookie has a session
    if let Some(session_uuid) = id.identity() {

        // Session ID is a valid UUID
        if let Ok(session_id) = Uuid::parse_str(
            &session_uuid
        ) {

            Ok(
                AdminRequest {
                    session: session_id,
                    lang_code: lang_code,
                }
            )

        // TODO: "Session ID is not a valid UUID."
        } else {

            // Delete cookie
            id.forget();

            // Redirect to login page
            Err(
                HttpResponse::Found()
                    .header("Location", ra_login(&lang_code))
                    .finish()
            )

        }

    // No session
    // TODO: "You need to login first."
    } else {

        // Redirect to login page
        Err(
            HttpResponse::Found()
                .header("Location", ra_login(&lang_code))
                .finish()
        )

    }

}

