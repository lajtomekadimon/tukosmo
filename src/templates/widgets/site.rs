use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{WebsiteDataDB, RouteDB};
use crate::templates::widgets::{
    header::Header,
    navigation::Navigation,
    widget::Widget,
    footer::Footer,
    languages::Languages,
};


markup::define! {
    Site<'a, BodyContent: markup::Render>(
        content: BodyContent,
        routes: &'a Vec<RouteDB>,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
    ) {
        @Languages {
            routes: routes,
            data: data,
            t: t,
        }

        div[
            id = "site",
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
                    t: t,
                }
            }

            div[
                class = "site-content",
            ] {
                div [
                    class = "site-container",
                ] {
                    div[
                        class = "site-content-inside",
                    ] {
                        main[
                            class = "site-main",
                        ] {
                            @content
                        }

                        aside[class = "site-sidebar"] {
                            @Widget {}
                        }
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

