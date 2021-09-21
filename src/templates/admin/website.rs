use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::types::DataDB;


markup::define! {
    Website<'a>(
        title: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang: &data.lang,
            content: AdminPanel {
                content: Content {},
                current_page: "website",
                data: data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

