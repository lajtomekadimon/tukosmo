use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::data::DataDB;


markup::define! {
    Tukosmo<'a>(
        title: &'a str,
        lang_code: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {},
                current_page: "tukosmo",
                lang_code: lang_code,
                data: data,
            },
        }
    }

    Content() {
        "Bla"
    }
}

