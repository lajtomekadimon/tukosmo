use markup;

use crate::templates::widgets::header::Header;
use crate::templates::widgets::navigation::Navigation;
use crate::templates::widgets::footer::Footer;


markup::define! {
    Layout<'a, BodyContent: markup::Render>(
        title: &'a str,
        lang_code: &'a str,
        content: BodyContent,
    ) {
        @markup::doctype()
        html[
            lang = lang_code
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
            @Header {}

            @Navigation {}

            // site-content
            div {
                // container {}
                div {
                    // site-main
                    main {
                        // List of blog posts
                        //section {}
                        //section {}
                        //section {}

                        // Blog post / page
                        //article {}
                        
                        @content
                    }

                    // site-sidebar
                    aside {}
                }
            }

            @Footer {}
        }
    }
}

