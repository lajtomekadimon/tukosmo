use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::website::WebsiteAResponse;


markup::define! {
    Website<'a>(
        title: &'a str,
        q: &'a WebsiteAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "website",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

