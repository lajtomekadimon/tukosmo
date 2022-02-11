use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{WebsiteDataDB, RouteDB};


markup::define! {
    Languages<'a>(
        routes: &'a Vec<RouteDB>,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
    ) {
        div[
            id = "languages",
            class = "languages",
        ] {
            div[
                id = "languages-bg",
                class = "languages-bg",
            ] {}
            div[class = "languages-content"] {
                button [
                    id = "languages-close",
                    class = "languages-close",
                    title = &t.close_k_verb,
                ] {}
                h3 {
                    @t.select_a_language
                }

                ul {
                    @for route in routes.iter() {
                        li {
                            a[
                                href = &route.route,
                            ] {
                                @route.lang.name
                                @if data.lang.code != route.lang.code {
                                    " ("
                                    @route.lang.original_name
                                    ")"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

