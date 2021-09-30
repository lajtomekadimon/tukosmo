use actix_web::HttpRequest;
use actix_identity::Identity;
use uuid::Uuid;

use crate::database::types::WebsiteRequest;


pub fn user_request(
    req: HttpRequest,
    id: Identity,
) -> WebsiteRequest {

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

    WebsiteRequest {
        session: maybe_session_id,
        lang_code: lang_code,
    }

}

