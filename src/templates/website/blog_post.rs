use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post::Post;
use crate::database::s_post_by_lang_permalink::PostDB;


markup::define! {
    BlogPost<'a>(
        title: &'a str,
        lang_code: &'a str,
        post: &'a PostDB,
    ) {
        @Layout {
            title: title,
            lang_code: lang_code,
            content: Post {
                lang_code: lang_code,
                post: &post,
            }
        }
    }
}

