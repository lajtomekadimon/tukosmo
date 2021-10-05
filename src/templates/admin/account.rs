use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::account::AccountAResponse;


markup::define! {
    Account<'a>(
        title: &'a str,
        q: &'a AccountAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "account",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

