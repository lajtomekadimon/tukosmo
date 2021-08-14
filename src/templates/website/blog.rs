use markup;

use crate::templates::layout::Layout;
use crate::templates::widgets::post_list::PostList;


markup::define! {
    Blog<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @Layout {
            title: title,
            lang_code: lang_code,
            content: Content {
                lang_code: lang_code,
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
    ) {
        @PostList {
            lang_code: lang_code,
        }
    }
}

