use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::templates::website::blog_post::BlogPost;
use crate::i18n::current_language::current_language;
use crate::database::s_post_by_lang_permalink::s_post_by_lang_permalink;


pub async fn blog_post(
    req: HttpRequest,
) -> impl Responder {
    let permalink_value: String = req.match_info()
        .get("permalink").unwrap().parse().unwrap();

    if let Some(lang) = current_language(req) {

        if let Some(post) = s_post_by_lang_permalink(
            lang.id.clone(),
            permalink_value
        ) {

            let html = BlogPost {
                title: &format!(
                    "{a} - {b}",
                    a = post.title,
                    b = "MyExample"
                ),
                lang: &lang,
                post: &post,
            };

            HttpResponse::Ok().body(html.to_string())
            
        } else {

            HttpResponse::Ok().body("Error 404")  // TODO

        }

    } else {

        HttpResponse::Ok().body("Error 404")  // TODO

    }
}

