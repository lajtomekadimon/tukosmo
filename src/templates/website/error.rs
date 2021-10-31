use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::error::ErrorWResponse;


markup::define! {
    Error<'a>(
        title: &'a str,
        q: &'a ErrorWResponse,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {
                    t: t,
                },
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        t: &'a TranslateI18N,
    ) {
        article[
            class = "error",
        ] {
            div[
                class = "error-content",
            ] {
                div[
                    class = "error-header",
                ] {
                    div[
                        class = "error-title",
                    ] {
                        h1[
                            class = "error-title",
                        ] {
                            @t.error
                        }
                    }
                }

                div[
                    class = "error-body",
                ] {
                    ""
                }
            }
        }
    }
}

