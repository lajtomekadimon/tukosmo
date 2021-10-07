use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::statistics::StatisticsAResponse;


markup::define! {
    Statistics<'a>(
        title: &'a str,
        q: &'a StatisticsAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "statistics",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

