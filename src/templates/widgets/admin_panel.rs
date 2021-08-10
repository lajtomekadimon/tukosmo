use markup;

use crate::templates::widgets::admin_sidebar::AdminSidebar;
use crate::templates::widgets::admin_navbar::AdminNavbar;


markup::define! {
    AdminPanel<'a, BodyContent: markup::Render>(
        content: BodyContent,
        current_page: &'a str,
    ) {
        @AdminNavbar {}

        div[class = "container"] {
            div[class = "columns"] {
                div[class = "column is-3"] {
                    @AdminSidebar {
                        current_page: current_page,
                    }
                }
                div[class = "column is-9"] {
                    @content
                }
            }
        }
    }
}

