use markup;

use crate::handlers::admin::tukosmo_get::AgoTukosmo;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Tukosmo<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoTukosmo,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    t: t,
                },
                current_page: "tukosmo",
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        t: &'a TranslateI18N,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.tukosmo
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

