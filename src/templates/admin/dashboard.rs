use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::dashboard::DashboardAResponse;


markup::define! {
    Dashboard<'a>(
        title: &'a str,
        q: &'a DashboardAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                },
                current_page: "dashboard",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a DashboardAResponse,
    ) {
        section[class = "hero is-info welcome is-small"] {
            div[class = "hero-body"] {
                div[class = "container"] {
                    h1[class = "title"] {
                        {&t(
                            "Hello, {name}.",
                            &q.data.lang.code
                        ).replace("{name}", &q.data.userd.name)}
                    }
                    h2[class = "subtitle"] {
                        {&t(
                            "I hope you are having a great day!",
                            &q.data.lang.code
                        )}
                    }
                }
            }
        }
    }
}

