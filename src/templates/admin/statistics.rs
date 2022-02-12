use markup;

use crate::handlers::admin::statistics_get::AgoStatistics;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Statistics<'a>(
        domain: &'a str,
        title: &'a str,
        q: &'a AgoStatistics,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            domain: domain,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {},
                current_page: "statistics",
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

