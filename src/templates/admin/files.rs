use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::data::DataDB;


markup::define! {
    Files<'a>(
        title: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang: &data.lang,
            content: AdminPanel {
                content: Content {},
                current_page: "files",
                data: data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

