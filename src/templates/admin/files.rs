use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::handlers::admin::files::FilesAResponse;


markup::define! {
    Files<'a>(
        title: &'a str,
        q: &'a FilesAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {},
                current_page: "files",
                data: &q.data,
            },
        }
    }

    Content() {
        "Coming soon!"
    }
}

