use markup;

use crate::templates::layout::Layout;


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
        div { "Test 1" }
    }
}

