use markup;

use crate::i18n::t::t;
use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::error::ErrorWResponse;


markup::define! {
    Error<'a>(
        title: &'a str,
        q: &'a ErrorWResponse,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {
                    q: q,
                },
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a ErrorWResponse,
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
                            {&t("Error", &q.data.lang.code)}
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

