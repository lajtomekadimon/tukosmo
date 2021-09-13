use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post_list::PostList;
use crate::database::data::{CurrentLanguageDB, PostDB};


markup::define! {
    Blog<'a>(
        title: &'a str,
        lang: &'a CurrentLanguageDB,
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
        lang: &'a CurrentLanguageDB,
        posts: &'a Vec<PostDB>,
    ) {
        @PostList {
            lang: lang,
            posts: posts,
        }
    }
}

