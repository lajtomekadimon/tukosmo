use markup;

use crate::handlers::website::error_get::WgoError;
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    website_layout::WebsiteLayout,
    widgets::site::Site,
};


markup::define! {
    Error<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a WgoError,
        t: &'a TranslateI18N,
        e: &'a ErrorDB,
    ) {
        @WebsiteLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            t: t,
            routes: &q.routes,
            og_title: &"ERROR {code}".replace("{code}", &e.code),
            og_description: "",
            og_image: "",
            og_article: &None,
            content: Site {
                content: Content {
                    e: e,
                },
                routes: &q.routes,
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

