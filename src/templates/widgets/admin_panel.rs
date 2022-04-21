use markup;

use crate::config::global::Config;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{
    AdminDataDB,
    RouteDB,
};
use crate::templates::widgets::{
    admin_sidebar::AdminSidebar,
    admin_navbar::AdminNavbar,
    admin_languages::AdminLanguages,
};


markup::define! {
    AdminPanel<'a, BodyContent: markup::Render>(
        content: BodyContent,
        current_page: &'a str,
        codename: &'a str,
        config: &'a Config,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
        routes: &'a Vec<RouteDB>,
    ) {
        @AdminLanguages {
            routes: routes,
            data: data,
            t: t,
        }

        @AdminNavbar {
            codename: codename,
            data: data,
            t: t,
        }

        div[class = "columns"] {
            div[class = "column is-2"] {
                @AdminSidebar {
                    current_page: current_page,
                    data: data,
                    t: t,
                    blog_enabled: &(config.modules.blog.enabled == "yes"),
                    gallery_enabled: &(
                        config.modules.gallery.enabled == "yes"
                    ),
                    faq_enabled: &(config.modules.faq.enabled == "yes"),
                    payments_enabled: &(
                        config.modules.payments.enabled == "yes"
                    ),
                    subscriptions_enabled: &(
                        config.modules.subscriptions.enabled == "yes"
                    ),
                    shop_enabled: &(config.modules.shop.enabled == "yes"),
                }
            }
            div[class = "column is-10"] {
                @content
            }
        }
    }
}

