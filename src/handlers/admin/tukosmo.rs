use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use uuid::Uuid;

use crate::i18n::t::t;
use crate::database::s_user_by_session::s_user_by_session;
use crate::templates::admin::tukosmo::Tukosmo;


pub async fn tukosmo(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {
    let lang_code: String = req.match_info().get("lang").unwrap().parse().unwrap();

    let login_route = "/{lang}/admin/login".replace("{lang}", &lang_code);

    // Cookie has a session
    if let Some(session_uuid) = id.identity() {

        if let Ok(session_id) = Uuid::parse_str(
            &session_uuid
        ) {

            // Session is active
            if let Ok(_user_id) = s_user_by_session(session_id) {

                let html = Tukosmo {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("Tukosmo", &lang_code),
                        b = &t("Tukosmo Admin Panel", &lang_code)
                    ),
                    lang_code: &lang_code,
                };

                HttpResponse::Ok().body(html.to_string())

            // Session has expired
            // TODO: "Your session has expired."
            } else {

                // Delete cookie
                id.forget();

                // Redirect to login
                HttpResponse::Found()
                    .header("Location", login_route)
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

