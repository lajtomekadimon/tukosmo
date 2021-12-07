use markup;

use crate::database::types::{WebsiteDataDB, RouteDB};
use crate::templates::widgets::open_graph_meta::{OpenGraphMeta, ArticleOG};


markup::define! {
    WebsiteLayout<'a, BodyContent: markup::Render>(
        title: &'a str,
        data: &'a WebsiteDataDB,
        routes: &'a Vec<RouteDB>,
        og_title: &'a str,
        og_description: &'a str,
        og_image: &'a str,
        og_article: &'a Option<ArticleOG>,
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
                    content = og_description,
                ];
                /*
                meta[
                    name = "author",
                    content = "",
                ];
                */

                link[
                    rel = "apple-touch-icon",
                    sizes = "57x57",
                    href = "/static/favicon/apple-icon-57x57.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "60x60",
                    href = "/static/favicon/apple-icon-60x60.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "72x72",
                    href = "/static/favicon/apple-icon-72x72.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "76x76",
                    href = "/static/favicon/apple-icon-76x76.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "114x114",
                    href = "/static/favicon/apple-icon-114x114.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "120x120",
                    href = "/static/favicon/apple-icon-120x120.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "144x144",
                    href = "/static/favicon/apple-icon-144x144.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "152x152",
                    href = "/static/favicon/apple-icon-152x152.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "180x180",
                    href = "/static/favicon/apple-icon-180x180.png",
                ];

                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "32x32",
                    href = "/static/favicon/favicon-32x32.png",
                ];
                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "96x96",
                    href = "/static/favicon/favicon-96x96.png",
                ];
                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "16x16",
                    href = "/static/favicon/favicon-16x16.png",
                ];

                link[
                    rel = "manifest",
                    href = "/static/favicon/manifest.json",
                ];

                // Open Graph
                @OpenGraphMeta {
                    data: data,
                    routes: routes,
                    title: og_title,
                    description: og_description,
                    image: og_image,
                    article: og_article,
                }

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

