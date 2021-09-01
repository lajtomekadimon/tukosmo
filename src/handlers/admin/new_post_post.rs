use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::database::aw_new_post::aw_new_post;


#[derive(Deserialize)]
pub struct FormData {
    title: String,
    description: String,
    body: String,
    permalink: String,
    draft: Option<String>,
}

pub async fn new_post_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok((lang_code, user_id)) => {
            let title_value = (form.title).clone();
            let description_value = (form.description).clone();
            let body_value = (form.body).clone();
            let permalink_value = (form.permalink).clone();
            let is_draft: bool = match (form.draft).clone() {
                Some(_) => true,
                None => false,
            };

            if let Ok(_post_trans_id) = aw_new_post(
                0,  // new
                lang_code.clone(),
                title_value,
                description_value,
                body_value,
                permalink_value,
                user_id,
                is_draft,
            ) {
                let redirect_route = "/{lang}/admin/posts".replace("{lang}", &lang_code);

                HttpResponse::Found()
                    .header("Location", redirect_route)
                    .finish()
            } else {
                let redirect_route = "/{lang}/admin/new_post".replace("{lang}", &lang_code);
                // TODO: Show what failed in the template!

                HttpResponse::Found()
                    .header("Location", redirect_route)
                    .finish()
            }
        }

        Err(r) => {r}

    }

}

