use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{
    AdminDataDB,
    RouteDB,
};
use crate::templates::widgets::{
    admin_sidebar::AdminSidebar,
    admin_navbar::AdminNavbar,
    admin_languages::AdminLanguages,
};


markup::define! {
    AdminPanel<'a, BodyContent: markup::Render>(
        content: BodyContent,
        current_page: &'a str,
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
        routes: &'a Vec<RouteDB>,
    ) {
        @AdminLanguages {
            routes: routes,
            data: data,
            t: t,
        }

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

