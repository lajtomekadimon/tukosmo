use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post_list::PostList;
use crate::database::data::CurrentLanguageDB;


markup::define! {
    Blog<'a>(
        title: &'a str,
        lang: &'a CurrentLanguageDB,
    ) {
        @Layout {
            title: title,
            lang: &lang,
            content: Content {
                lang: lang,
            },
        }
    }

    Content<'a>(
        lang: &'a CurrentLanguageDB
    ) {
        @PostList {
            lang: lang,
        }
    }
}

