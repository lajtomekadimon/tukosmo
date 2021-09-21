use actix_web::{HttpRequest, HttpResponse};
use actix_identity::Identity;
use uuid::Uuid;

use crate::database::aww_website_handler::aww_website_handler;
use crate::database::types::WebsiteDataDB;


pub fn website_handler(
    req: HttpRequest,
    id: Identity,
) -> Result<WebsiteDataDB, HttpResponse> {

    let lang_code: String = req.match_info()
        .get("lang").unwrap().parse().unwrap();

    let maybe_session_id: Option<Uuid>;
    // Cookie has a session
    if let Some(session_uuid) = id.identity() {
        // Session ID is a valid UUID
        if let Ok(session_id) = Uuid::parse_str(
            &session_uuid
        ) {
            maybe_session_id = Some(session_id);
        // Session ID is not a valid UUID
        } else {
            // Delete cookie
            id.forget();

            maybe_session_id = None;
        }
    // Cookie doesn't have a session
    } else {
        maybe_session_id = None;
    }

    if let Some(data) = aww_website_handler(
        maybe_session_id,
        lang_code,
    ) {

        Ok(data)

    } else {

        Err(
            HttpResponse::Ok().body("Error 404.")  // TODO
        )

    }

}

