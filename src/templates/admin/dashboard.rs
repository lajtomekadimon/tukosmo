use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::dashboard::DashboardAResponse;


markup::define! {
    Dashboard<'a>(
        title: &'a str,
        t: &'a TranslateI18N,
        q: &'a DashboardAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                },
                current_page: "dashboard",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a DashboardAResponse,
        t: &'a TranslateI18N,
    ) {
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

