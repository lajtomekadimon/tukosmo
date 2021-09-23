use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use uuid::Uuid;

use crate::handlers::website::website_handler::website_handler;
use crate::database::awa_login_post::awa_login_post;


#[derive(Deserialize)]
pub struct FormData {
    email: String,
    password: String,
}

pub async fn login_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match website_handler(req.clone(), id.clone()) {

        Ok(data) => {

            let email_value = (form.email).clone();
            let password_value = (form.password).clone();

            let user_agent_value = match req.headers().get("User-Agent") {
                Some(the_user_agent) => {
                    match the_user_agent.to_str() {
                        Ok(ua_value) => ua_value.to_string(),
                        _ => "unknown".to_string(),
                    }
                },
                None => "unknown".to_string(),
            };

            if let Ok(session_id) = awa_login_post(
                email_value,
                password_value,  // TODO: This is not encrypted!!!
                                 /* Currently, password check is done in the
                                  * database. It may be more secure to do this
                                  * in the web server, but the downside is that
                                  * instead of one query, we would need to do two
                                  * queries: one for the password check, and
                                  * another one for the new session.
                                  */
                user_agent_value,
            ) {

                let encode_buffer_value = &mut Uuid::encode_buffer();

                let session_id_up = session_id
                    .to_simple()
                    .encode_upper(encode_buffer_value);

                id.remember(session_id_up.to_owned());

                HttpResponse::Found()
                    .header(
                        "Location",
                        "/{lang}/admin/".replace("{lang}", &data.lang.code),
                    )
                    .finish()
                

            } else {

                // TODO: Email or password not correct
                HttpResponse::Found()
                    .header(
                        "Location",
                        "/{lang}/admin/login"
                            .replace("{lang}", &data.lang.code),
                    )
                    .finish()

            }

        }

        Err(r) => r

    }

}
