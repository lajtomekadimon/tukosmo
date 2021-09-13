use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use bcrypt::verify;
use uuid::Uuid;

use crate::i18n::current_language::current_language;
use crate::database::s_user_password_by_email::s_user_password_by_email;
use crate::database::i_session_by_email::i_session_by_email;


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
    if let Some(lang) = current_language(req.clone()) {

        let email_value = (form.email).clone();
        let password_value = (form.password).clone();

        let login_route = "/{lang}/admin/login".replace("{lang}", &lang.code);

        let redirect_dir;

        if let Ok(selected_password_value) = s_user_password_by_email(
            email_value.clone()
        ) {
            if let Ok(password_is_correct) = verify(
                password_value,
                &selected_password_value
            ) {
                // Login!!
                if password_is_correct {

                    if let Some(the_user_agent) = req.headers().get("User-Agent") {

                        if let Ok(user_agent_value) = the_user_agent.to_str() {

                            if let Ok(session_id) = i_session_by_email(
                                email_value.clone(),
                                user_agent_value.to_string()
                            ) {

                                let encode_buffer_value = &mut Uuid::encode_buffer();

                                let session_id_up = session_id
                                    .to_simple()
                                    .encode_upper(encode_buffer_value);

                                id.remember(session_id_up.to_owned());

                                redirect_dir = "/{lang}/admin/".replace("{lang}", &lang.code);

                            } else {

                                redirect_dir = login_route;

                            }

                        } else {

                            redirect_dir = login_route;

                        }

                    } else {

                        redirect_dir = login_route;

                    }

                // ERROR: Password is not correct
                } else {

                    redirect_dir = login_route;

                }

            // ERROR: Bcrypt error (password is not correct?)
            } else {

                redirect_dir = login_route;

            }

        // ERROR: Database error
        } else {

            redirect_dir = login_route;

        }

        HttpResponse::Found()
            .header("Location", redirect_dir)
            .finish()

    } else {

        HttpResponse::Found()
            .header("Location", "/")  // TODO
            .finish()

    }
}
