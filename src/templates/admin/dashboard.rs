use markup;

use crate::handlers::admin::dashboard_get::AgoDashboard;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Dashboard<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        t: &'a TranslateI18N,
        q: &'a AgoDashboard,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                },
                current_page: "dashboard",
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoDashboard,
        t: &'a TranslateI18N,
    ) {
        /* TODO:
         * - Show Tukosmo version and available update
         * - Show server info and available update
         */
        section[class = "hero is-info welcome is-small"] {
            div[class = "hero-body"] {
                div[class = "container"] {
                    h1[class = "title"] {
                        @t.hello_user
                            .replace("{name}", &q.data.userd.name)
                    }
                    h2[class = "subtitle"] {
                        @t.i_hope_you_are_having_a_great_day
                    }
                }
            }
        }
    }
}

