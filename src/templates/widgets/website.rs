use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::widgets::header::Header;
use crate::templates::widgets::navigation::Navigation;
use crate::templates::widgets::sidebar::Sidebar;
use crate::templates::widgets::footer::Footer;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Website<'a, BodyContent: markup::Render>(
        content: BodyContent,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
    ) {
        div[
            id = "page",
            class = "site",
        ] {
            div[
                class = "site-top",
            ] {
                @Header {
                    data: data,
                }

                @Navigation {}
            }

            div[
                class = "site-content",
            ] {
                div [
                    class = "container",
                ] {
                    div[
                        class = "site-content-inside",
                    ] {
                        main[
                            class = "site-main",
                        ] {
                            @content
                        }

                        @Sidebar {}
                    }
                }
            }

            @Footer {
                data: data,
                t: t,
            }
        }
    }
}

