use markup;

use crate::database::types::{WebsiteDataDB, RouteDB};


markup::define! {
    WebsiteLayout<'a, BodyContent: markup::Render>(
        title: &'a str,
        data: &'a WebsiteDataDB,
        routes: &'a Vec<RouteDB>,
        content: BodyContent,
    ) {
        @markup::doctype()
        html[
            lang = &data.lang.code
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

                // i18n routes
                @for route in routes.iter() {
                    link[
                        rel = "alternate",
                        hreflang = &route.lang.code,
                        href = "https://tukosmo.org{route}"  // TODO: Domain!!
                            .replace("{route}", &route.route),
                    ];
                }

                // Styles
                /*link[
                    rel = "stylesheet",
                    href = "https://fonts.googleapis.com/css?family=Poppins",
                ];*/

                link[
                    rel = "stylesheet",
                    href = "https://cdn.jsdelivr.net/npm/\
                            eos-icons@latest/dist/css/eos-icons.css",
                ];


                link[
                    rel = "stylesheet",
                    href = "/static/bundle.css",
                ];
            }
            body {
                @content

                script[
                    src = "/static/bundle.js",
                ] {}
            }
        }
    }
}

