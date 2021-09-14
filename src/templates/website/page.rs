use markup;

use crate::templates::layout::Layout;
use crate::database::data::LanguageDB;


markup::define! {
    Page<'a>(
        title: &'a str,
        lang: &'a LanguageDB,
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

