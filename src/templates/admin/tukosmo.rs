use markup;

use crate::handlers::admin::tukosmo_get::AgoTukosmo;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
};


markup::define! {
    Tukosmo<'a>(
        title: &'a str,
        q: &'a AgoTukosmo,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                },
                current_page: "tukosmo",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoTukosmo,
        t: &'a TranslateI18N,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.tukosmo

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }
            }

            p {
                "Version: 0.0.1"
            }

            /*
            button[
                class = "button",
                disabled = true,
            ] {
                @t.update
            }
            */
        }
    }
}

