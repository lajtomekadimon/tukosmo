use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::types::AdminDataDB;


markup::define! {
    Pages<'a>(
        title: &'a str,
        data: &'a AdminDataDB,
    ) {
        @AdminLayout {
            title: title,
            data: data,
            content: AdminPanel {
                content: Content {},
                current_page: "pages",
                data: data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

