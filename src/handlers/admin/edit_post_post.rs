use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use serde::Deserialize;

use crate::handlers::admin::admin_handler::admin_handler;
use crate::database::awa_edit_post_post::awa_edit_post_post;


#[derive(Deserialize)]
pub struct FormData {
    id: i64,
    title: String,
    description: String,
    body: String,
    permalink: String,
    draft: Option<String>,
    deleted: Option<String>,
}

pub async fn edit_post_post(
    req: HttpRequest,
    form: web::Form<FormData>,
    id: Identity,
) -> impl Responder {

    match admin_handler(req, id) {

        Ok(data) => {
            let post_id = (form.id).clone();
            let title_value = (form.title).clone();
            let description_value = (form.description).clone();
            let body_value = (form.body).clone();
            let permalink_value = (form.permalink).clone();
            let is_draft: bool = match (form.draft).clone() {
                Some(_) => true,
                None => false,
            };
            let is_deleted: bool = match (form.deleted).clone() {
                Some(_) => true,
                None => false,
            };

            if let Ok(_post_trans_id) = awa_edit_post_post(
                post_id,
                data.lang.code.clone(),
                title_value,
                description_value,
                body_value,
                permalink_value,
                is_draft,
                is_deleted,
                data.userd.id,
            ) {
                let redirect_route = "/{lang}/admin/posts"
                    .replace("{lang}", &data.lang.code);

                HttpResponse::Found()
                    .header("Location", redirect_route)
                    .finish()
            } else {
                let redirect_route = "/{lang}/admin/edit_post?id={id}"
                    .replace("{lang}", &data.lang.code)
                    .replace("{id}", &post_id.to_string());
                // TODO: Show what failed in the template!

                HttpResponse::Found()
                    .header("Location", redirect_route)
                    .finish()
            }
        }

        Err(r) => {r}

    }

}

