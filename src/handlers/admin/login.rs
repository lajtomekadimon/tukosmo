use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use uuid::Uuid;

use crate::i18n::current_language::current_language;
use crate::i18n::t::t;
use crate::templates::admin::login::Login;
use crate::database::aw_login::aw_login;


pub async fn login(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    if let Some(lang) = current_language(req) {

        let html = Login {
            title: &format!(
                "{a} - {b}",
                a = &t("Login [noun]", &lang.code),
                b = &t("Tukosmo Admin Panel", &lang.code)
            ),
            lang: &lang,
        };

        // Cookie has a session
        if let Some(session_uuid) = id.identity() {

            if let Ok(session_id) = Uuid::parse_str(
                &session_uuid
            ) {

                // Session exists or existed
                if let Ok(is_correct) = aw_login(session_id) {

                    // Session is active
                    if is_correct {

                        let dashboard_route = "/{lang}/admin/"
                            .replace("{lang}", &lang.code);

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

                // TODO: Database error
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

        // Cookie has no session
        } else {

            HttpResponse::Ok().body(html.to_string())

        }

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }

}

