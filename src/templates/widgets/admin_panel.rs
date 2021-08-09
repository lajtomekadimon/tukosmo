use markup;

use crate::templates::widgets::admin_sidebar::AdminSidebar;
use crate::templates::widgets::admin_navbar::AdminNavbar;
use crate::templates::widgets::admin_breadcrumb::AdminBreadcrumb;


markup::define! {
    AdminPanel<BodyContent: markup::Render>(
        content: BodyContent,
    ) {
        @AdminNavbar {}

        div[class = "container"] {
            div[class = "columns"] {
                div[class = "column is-3"] {
                    @AdminSidebar {}
                }
                div[class = "column is-9"] {
                    @AdminBreadcrumb {}

                    @content
                }
            }
        }
    }
}

