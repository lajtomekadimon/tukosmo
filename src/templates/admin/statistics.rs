use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::data::DataDB;


markup::define! {
    Statistics<'a>(
        title: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang: &data.lang,
            content: AdminPanel {
                content: Content {},
                current_page: "statistics",
                data: data,
            },
        }
    }

    Content() {
        "Bla"
    }
}

