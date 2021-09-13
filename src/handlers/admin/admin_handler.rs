use actix_web::{HttpRequest, HttpResponse};
use actix_identity::Identity;
use uuid::Uuid;

use crate::i18n::current_language::current_language;
use crate::database::s_user_by_session_lang::s_user_by_session_lang;
use crate::database::data::UserDB;


pub fn admin_handler(
    req: HttpRequest,
    id: Identity,
) -> Result<(String, UserDB), HttpResponse> {
    if let Some(lang_code) = current_language(req) {

        let login_route = "/{lang}/admin/login".replace("{lang}", &lang_code);

        // Cookie has a session
        if let Some(session_uuid) = id.identity() {

            if let Ok(session_id) = Uuid::parse_str(
                &session_uuid
            ) {

                // Session is active
                if let Some(user) = s_user_by_session_lang(
                    session_id,
                    lang_code.clone(),
                ) {

                    Ok((lang_code, user))

                // Session has expired
                // TODO: "Your session has expired."
                } else {

                    // Delete cookie
                    id.forget();

                    // Redirect to login
                    Err(HttpResponse::Found()
                        .header("Location", login_route)
                        .finish()
                    )

                }

            // TODO: "Session ID is not a valid UUID."
            } else {

                // Delete cookie
                id.forget();

                // Redirect to login
                Err(HttpResponse::Found()
                    .header("Location", login_route)
                    .finish()
                )

            }

        // No session
        // TODO: "You need to login first."
        } else {

            // Redirect to login
            Err(HttpResponse::Found()
                .header("Location", login_route)
                .finish()
            )

        }

    } else {

        Err(HttpResponse::Ok().body("Error 404"))  // TODO

    }
}

