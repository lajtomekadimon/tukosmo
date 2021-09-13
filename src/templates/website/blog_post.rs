use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post::Post;
use crate::database::data::CurrentLanguageDB;
use crate::database::data::PostDB;


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

