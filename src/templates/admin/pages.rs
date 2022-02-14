use markup;

use crate::handlers::admin::pages_get::AgoPages;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Pages<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoPages,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {},
                current_page: "pages",
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

