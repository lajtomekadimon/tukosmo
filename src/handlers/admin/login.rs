use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use uuid::Uuid;

use crate::i18n::current_language::current_language;
use crate::i18n::t::t;
use crate::templates::admin::login::Login;
use crate::database::s_user_by_session::s_user_by_session;


pub async fn login(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    if let Some(lang_code) = current_language(req) {

        let html = Login {
            title: &format!(
                "{a} - {b}",
                a = &t("Login [noun]", &lang_code),
                b = &t("Tukosmo Admin Panel", &lang_code)
            ),
            lang_code: &lang_code,
        };

        // Cookie has a session
        if let Some(session_uuid) = id.identity() {

            if let Ok(session_id) = Uuid::parse_str(
                &session_uuid
            ) {

                // Session is active
                if let Ok(_user_id) = s_user_by_session(session_id) {

                    let dashboard_route = "/{lang}/admin/"
                        .replace("{lang}", &lang_code);

                    HttpResponse::Found()
                        .header("Location", dashboard_route)
                        .finish()

                // Session has expired
                // TODO: "Your session has expired."
                } else {

                    // Delete cookie
                    id.forget();

                    HttpResponse::Ok().body(html.to_string())

                }

            // TODO: "Session ID is not a valid UUID."
                } else {

                // Delete cookie
                id.forget();

                HttpResponse::Ok().body(html.to_string())

            }

        // No session
        // TODO: "You need to login first."
        } else {

            HttpResponse::Ok().body(html.to_string())

        }

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }

}

