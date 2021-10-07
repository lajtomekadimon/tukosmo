use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::users::UsersAResponse;


markup::define! {
    Users<'a>(
        title: &'a str,
        q: &'a UsersAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "users",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

