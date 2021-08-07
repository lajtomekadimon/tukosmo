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
            content: Content {},
        }
    }

    Content() {
        @PostList {}
    }
}

