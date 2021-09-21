use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::i18n::t::t;
use crate::templates::admin::login::Login;
use crate::handlers::website::website_handler::website_handler;
use crate::database::types::{AdminDataDB, UserDB};


pub async fn login(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match website_handler(req, id.clone()) {

        Ok(data) => {

            if let Some(_user) = data.userd {

                let dashboard_route = "/{lang}/admin/"
                    .replace("{lang}", &data.lang.code);

                HttpResponse::Found()
                    .header("Location", dashboard_route)
                    .finish()

            } else {

                // Delete cookie
                id.forget();

                let html = Login {
                    title: &format!(
                        "{a} - {b}",
                        a = &t("Login [noun]", &data.lang.code),
                        b = &t("Tukosmo Admin Panel", &data.lang.code)
                    ),
                    data: &AdminDataDB {
                        userd: UserDB {
                            id: 0,
                            email: "".to_string(),
                            name: "".to_string(),
                        },
                        lang: data.lang,
                        languages: data.languages,
                    },
                };

                HttpResponse::Ok().body(html.to_string())

            }

        }

        Err(r) => r

    }

}

