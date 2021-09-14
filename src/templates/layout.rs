use markup;

use crate::templates::widgets::header::Header;
use crate::templates::widgets::navigation::Navigation;
use crate::templates::widgets::sidebar::Sidebar;
use crate::templates::widgets::footer::Footer;
use crate::database::data::LanguageDB;


markup::define! {
    Layout<'a, BodyContent: markup::Render>(
        title: &'a str,
        lang: &'a LanguageDB,
        content: BodyContent,
    ) {
        @markup::doctype()
        html[
            lang = &lang.code
        ] {
            head {
                meta[charset = "utf-8"];
                meta[
                    name = "viewport",
                    content = "width=device-width, initial-scale=1.0",
                ];

                title { @title }

                meta[
                    name = "description",
                    content = "",
                ];
                meta[
                    name = "author",
                    content = "",
                ];

                // These are many
                /*link[
                    rel = "apple-touch-icon",
                    sizes = "",
                    href = "",
                ];*/

                // These are many
                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "16x16",
                    href = "/static/favicon/favicon-16x16.png",
                ];

                /*link[
                    rel = "manifest",
                    href = "",
                ];*/

                // Styles
                /*link[
                    rel = "stylesheet",
                    href = "https://fonts.googleapis.com/css?family=Poppins",
                ];*/

                link[
                    rel = "stylesheet",
                    href = "https://cdn.jsdelivr.net/npm/eos-icons@latest/dist/css/eos-icons.css",
                ];


                link[
                    rel = "stylesheet",
                    href = "/static/bundle.css",
                ];
            }
            body {
                @Interface {
                    content: content
                }

                /*
                script[
                    src = "",
                ] {}
                */
            }
        }
    }

    Interface<BodyContent: markup::Render>(
        content: BodyContent,
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

