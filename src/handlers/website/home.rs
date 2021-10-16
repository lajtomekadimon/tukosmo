use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::handlers::website::blog::{BlogWRequest, BlogWResponse};
use crate::handlers::website::user_request::user_request;
use crate::i18n::t::t;
use crate::templates::website::blog::Blog;
use crate::database::query_db::query_db;


pub async fn home(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req, id);

    match query_db(
        BlogWRequest {
            req: user_req.clone(),
            //results_per_page: results_per_page,
            //page: current_page,
        },
    ) {

        Ok(row) => {

            let q: BlogWResponse = row.get(0);

            let html = Blog {
                title: &format!(
                    "{a} - {b}",
                    a = &t("Blog", &q.data.lang.code),
                    b = "MyExample"
                ),
                q: &q,
            };

            HttpResponse::Ok().body(html.to_string())

        }

        Err(e) => {
            println!("{}", e);
            // TODO
            HttpResponse::Found()
                .header("Location", "/{lang}/error"
                    .replace("{lang}", &user_req.lang_code)
                )
                .finish()
        },

    }

}
