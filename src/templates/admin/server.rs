use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::server::ServerAResponse;


markup::define! {
    Server<'a>(
        title: &'a str,
        q: &'a ServerAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "server",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

