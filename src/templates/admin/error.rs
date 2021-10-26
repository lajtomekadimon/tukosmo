use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::error::ErrorAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    Error<'a>(
        title: &'a str,
        q: &'a ErrorAResponse,
        e: &'a ErrorDB,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    e: e,
                },
                current_page: "error",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a ErrorAResponse,
        e: &'a ErrorDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {"ERROR {code}".replace("{code}", &e.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/error?code={code}"
                            .replace("{code}", &e.code),
                        data: &q.data,
                    }
                }
            }

            div[class = "content"] {
                p {
                    @e.message
                }
            }
        }
    }
}

