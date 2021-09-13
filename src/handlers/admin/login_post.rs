use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;
use uuid::Uuid;

use crate::i18n::current_language::current_language;
use crate::database::aw_login_post::aw_login_post;


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

        let login_route = "/{lang}/admin/login".replace("{lang}", &lang.code);

        let redirect_dir;

        let email_value = (form.email).clone();
        let password_value = (form.password).clone();

        let mut user_agent_value = "unknown".to_string();
        if let Some(the_user_agent) = req.headers().get("User-Agent") {
            if let Ok(ua_value) = the_user_agent.to_str() {
                user_agent_value = ua_value.to_string();
            }
        }

        if let Ok(session_id) = aw_login_post(
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

            redirect_dir = "/{lang}/admin/".replace("{lang}", &lang.code);
            

        } else {

            // Email or password not correct
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
