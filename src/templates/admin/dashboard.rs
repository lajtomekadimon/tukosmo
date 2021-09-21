use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::types::AdminDataDB;


markup::define! {
    Dashboard<'a>(
        title: &'a str,
        data: &'a AdminDataDB,
    ) {
        @AdminLayout {
            title: title,
            data: data,
            content: AdminPanel {
                content: Content {
                    data: data,
                },
                current_page: "dashboard",
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a AdminDataDB,
    ) {
        section[class = "hero is-info welcome is-small"] {
            div[class = "hero-body"] {
                div[class = "container"] {
                    h1[class = "title"] {
                        {&t(
                            "Hello, {name}.",
                            &data.lang.code
                        ).replace("{name}", &data.userd.name)}
                    }
                    h2[class = "subtitle"] {
                        {&t(
                            "I hope you are having a great day!",
                            &data.lang.code
                        )}
                    }
                }
            }
        }
    }
}

