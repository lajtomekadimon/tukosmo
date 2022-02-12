use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{AdminDataDB, RouteDB};
use crate::handlers::admin::languages_get::ra_languages;


markup::define! {
    AdminLanguages<'a>(
        routes: &'a Vec<RouteDB>,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
    ) {
        div[
            id = "languages",
            class = "modal",
        ] {
            div[
                id = "languages-bg",
                class = "modal-background",
            ] {}

            div[class = "modal-card"] {
                header[class = "modal-card-head"] {
                    p[class = "modal-card-title"] {
                        @t.select_a_language
                    }
                    button[
                        id = "languages-close",
                        class = "delete",
                        title = &t.close_k_verb,
                    ] {}
                }

                section[class = "modal-card-body"] {
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

                footer[class = "modal-card-foot"] {
                    a[
                        class = "button is-link",
                        href = ra_languages(&data.lang.code),
                    ] {
                        @t.see_languages
                    }
                }
            }
        }
    }
}

