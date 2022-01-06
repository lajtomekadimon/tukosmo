use markup;

use crate::handlers::admin::statistics_get::AgoStatistics;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Statistics<'a>(
        title: &'a str,
        q: &'a AgoStatistics,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {},
                current_page: "statistics",
                data: &q.data,
                t: t,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

