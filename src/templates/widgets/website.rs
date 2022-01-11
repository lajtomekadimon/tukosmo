use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{WebsiteDataDB, RouteDB};
use crate::templates::widgets::{
    header::Header,
    navigation::Navigation,
    sidebar::Sidebar,
    footer::Footer,
};


markup::define! {
    Website<'a, BodyContent: markup::Render>(
        content: BodyContent,
        routes: &'a Vec<RouteDB>,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
    ) {
        div[
            id = "site-languages",
            class = "site-languages",
        ] {
            div[
                id = "site-languages-bg",
                class = "site-languages-bg",
            ] {}
            div[class = "site-languages-content"] {
                button [
                    id = "close-site-languages",
                    class = "close-site-languages",
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
        div[
            id = "page",
            class = "site",
        ] {
            div[
                class = "site-top",
            ] {
                @Header {
                    data: data,
                }

                @Navigation {
                    data: data,
                }
            }

            div[
                class = "site-content",
            ] {
                div [
                    class = "container",
                ] {
                    div[
                        class = "site-content-inside",
                    ] {
                        main[
                            class = "site-main",
                        ] {
                            @content
                        }

                        @Sidebar {}
                    }
                }
            }

            @Footer {
                t: t,
                data: data,
            }
        }
    }
}

