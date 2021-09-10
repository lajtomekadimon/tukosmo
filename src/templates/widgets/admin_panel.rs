use markup;

use crate::templates::widgets::admin_sidebar::AdminSidebar;
use crate::templates::widgets::admin_navbar::AdminNavbar;
use crate::database::s_user_by_session_lang::UserDB;


markup::define! {
    AdminPanel<'a, BodyContent: markup::Render>(
        content: BodyContent,
        current_page: &'a str,
        lang_code: &'a str,
        user: &'a UserDB,
    ) {
        @AdminNavbar {
            lang_code: lang_code,
            user: user,
        }

        div[class = "container"] {
            div[class = "columns"] {
                div[class = "column is-3"] {
                    @AdminSidebar {
                        current_page: current_page,
                        lang_code: lang_code,
                    }
                }
                div[class = "column is-9"] {
                    @content
                }
            }
        }
    }
}

