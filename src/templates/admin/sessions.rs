use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::sessions::SessionsAResponse;


markup::define! {
    Sessions<'a>(
        title: &'a str,
        q: &'a SessionsAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "sessions",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

