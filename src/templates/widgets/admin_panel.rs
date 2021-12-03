use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::widgets::admin_sidebar::AdminSidebar;
use crate::templates::widgets::admin_navbar::AdminNavbar;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminPanel<'a, BodyContent: markup::Render>(
        content: BodyContent,
        current_page: &'a str,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
    ) {
        @AdminNavbar {
            data: data,
            t: t,
        }

        //div[class = "container"] {
            div[class = "columns"] {
                div[class = "column is-2"] {
                    @AdminSidebar {
                        current_page: current_page,
                        data: data,
                        t: t,
                    }
                }
                div[class = "column is-10"] {
                    @content
                }
            }
        //}
    }
}

