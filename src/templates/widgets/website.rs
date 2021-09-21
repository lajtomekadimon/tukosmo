use markup;

use crate::templates::widgets::header::Header;
use crate::templates::widgets::navigation::Navigation;
use crate::templates::widgets::sidebar::Sidebar;
use crate::templates::widgets::footer::Footer;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Website<'a, BodyContent: markup::Render>(
        content: BodyContent,
        data: &'a WebsiteDataDB,
    ) {
        div[
            id = "page",
            class = "site",
        ] {
            div[
                class = "site-top",
            ] {
                @Header {}

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

            @Footer {}
        }
    }
}

