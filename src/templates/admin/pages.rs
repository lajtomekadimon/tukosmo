use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::pages::PagesAResponse;


markup::define! {
    Pages<'a>(
        title: &'a str,
        q: &'a PagesAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "pages",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

