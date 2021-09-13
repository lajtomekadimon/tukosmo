use markup;

use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::s_user_by_session_lang::UserDB;


markup::define! {
    Sessions<'a>(
        title: &'a str,
        lang_code: &'a str,
        user: &'a UserDB,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {},
                current_page: "sessions",
                lang_code: lang_code,
                user: user,
            },
        }
    }

    Content() {
        "Bla"
    }
}

