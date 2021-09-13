use markup;

use crate::templates::layout::Layout;
use crate::database::data::CurrentLanguageDB;


markup::define! {
    Page<'a>(
        title: &'a str,
        lang: &'a CurrentLanguageDB,
    ) {
        @Layout {
            title: title,
            lang: &lang,
            content: Content {},
        }
    }

    Content() {
        div { "Coming soon!" }
    }
}

