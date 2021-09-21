use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::templates::website::page::Page;
use crate::handlers::website::website_handler::website_handler;


pub async fn page(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match website_handler(req, id) {

        Ok(data) => {

            let html = Page {
                title: &format!(
                    "{a} - {b}",
                    a = "[page title]",
                    b = "MyExample"
                ),
                data: &data,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(r) => {r}

    }

}
