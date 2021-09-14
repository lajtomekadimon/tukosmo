use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post::Post;
use crate::database::data::{LanguageDB, PostDB};


markup::define! {
    BlogPost<'a>(
        title: &'a str,
        lang: &'a LanguageDB,
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

