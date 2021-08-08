use markup;

use crate::templates::widgets::admin_sidebar::AdminSidebar;
use crate::templates::widgets::admin_navbar::AdminNavbar;
use crate::templates::widgets::admin_breadcrumb::AdminBreadcrumb;


markup::define! {
    AdminLayout<'a, BodyContent: markup::Render>(
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

                link[
                    rel = "stylesheet",
                    href = "https://cdn.jsdelivr.net/npm/eos-icons@latest/dist/css/eos-icons.css",
                ];

                link[
                    rel = "stylesheet",
                    href = "/static/bundle.admin.css",
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

