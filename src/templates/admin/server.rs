use markup;

use crate::handlers::admin::server_get::AgoServer;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Server<'a>(
        domain: &'a str,
        title: &'a str,
        q: &'a AgoServer,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            domain: domain,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {},
                current_page: "server",
                data: &q.data,
                t: t,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

