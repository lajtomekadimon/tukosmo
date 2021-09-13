use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post::Post;
use crate::database::s_post_by_lang_permalink::PostDB;
use crate::database::data::CurrentLanguageDB;


markup::define! {
    BlogPost<'a>(
        title: &'a str,
        lang: &'a CurrentLanguageDB,
        post: &'a PostDB,
    ) {
        @Layout {
            title: title,
            lang: &lang,
            content: Post {
                lang: &lang,
                post: &post,
            }
        }
    }
}

