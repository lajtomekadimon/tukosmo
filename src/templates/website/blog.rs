use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post_list::PostList;
use crate::database::types::{LanguageDB, PostDB};


markup::define! {
    Blog<'a>(
        title: &'a str,
        lang: &'a LanguageDB,
        posts: &'a Vec<PostDB>,
    ) {
        @Layout {
            title: title,
            lang: &lang,
            content: Content {
                lang: lang,
                posts: posts,
            },
        }
    }

    Content<'a>(
        lang: &'a LanguageDB,
        posts: &'a Vec<PostDB>,
    ) {
        @PostList {
            lang: lang,
            posts: posts,
        }
    }
}

