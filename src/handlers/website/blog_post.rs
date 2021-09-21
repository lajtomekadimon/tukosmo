use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;

use crate::templates::website::blog_post::BlogPost;
use crate::handlers::website::website_handler::website_handler;
use crate::database::aww_blog_post::aww_blog_post;


pub async fn blog_post(
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    match website_handler(req.clone(), id) {

        Ok(data) => {

            let permalink_value: String = req.match_info()
                .get("permalink").unwrap().parse().unwrap();

            if let Some(post) = aww_blog_post(
                data.lang.id.clone(),
                permalink_value
            ) {

                let html = BlogPost {
                    title: &format!(
                        "{a} - {b}",
                        a = post.title,
                        b = "MyExample"
                    ),
                    data: &data,
                    post: &post,
                };

                HttpResponse::Ok().body(html.to_string())
                
            } else {

                HttpResponse::Ok().body("Error 404.")  // TODO

            }

        }

        Err(r) => {r}

    }

}
