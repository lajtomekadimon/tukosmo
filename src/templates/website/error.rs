use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_error::ErrorDB;
use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::error::ErrorWResponse;


markup::define! {
    Error<'a>(
        title: &'a str,
        q: &'a ErrorWResponse,
        t: &'a TranslateI18N,
        e: &'a ErrorDB,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {
                    e: e,
                },
                route: &"/error?code={code}".replace("{code}", &e.code),
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        e: &'a ErrorDB,
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
                            {"ERROR {code}".replace("{code}", &e.code)}
                        }
                    }
                }

                div[
                    class = "error-body",
                ] {
                    p {
                        @e.message
                    }
                }
            }
        }
    }
}

