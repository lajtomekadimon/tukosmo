use markup;

use crate::config::global::Config;
use crate::handlers::admin::error_get::AgoError;
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Error<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
        title: &'a str,
        q: &'a AgoError,
        t: &'a TranslateI18N,
        e: &'a ErrorDB,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    e: e,
                },
                current_page: "error",
                codename: codename,
                config: config,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        e: &'a ErrorDB,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                {"ERROR {code}".replace("{code}", &e.code)}
            }

            div[class = "content"] {
                p {
                    @e.message
                }
            }
        }
    }
}

