use markup;

use crate::files::static_files;
use crate::database::types::{WebsiteDataDB, RouteDB};
use crate::templates::widgets::{
    favicon_meta::FaviconMeta,
    open_graph_meta::{OpenGraphMeta, ArticleOG},
};


markup::define! {
    WebsiteLayout<'a, BodyContent: markup::Render>(
        domain: &'a str,
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

                // Favicon
                @FaviconMeta {}

                // Open Graph
                @OpenGraphMeta {
                    data: data,
                    routes: routes,
                    domain: domain,
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
                        href = "https://{domain}{route}"
                            .replace("{domain}", domain)
                            .replace("{route}", &route.route),
                    ];
                }

                // Styles
                /*link[
                    rel = "stylesheet",
                    href = "https://fonts.googleapis.com/css?family=Poppins",
                ];*/

                // TODO: Add it to Tukosmo's CSS
                link[
                    rel = "stylesheet",
                    href = "https://cdn.jsdelivr.net/npm/\
                            eos-icons@latest/dist/css/eos-icons.css",
                ];


                link[
                    rel = "stylesheet",
                    href = "/static/bundle.css",
                    href = static_files::CSS_WEBSITE,
                ];
            }
            body {
                @content

                // JavaScript
                script[src = static_files::JS_WEBSITE] {}
            }
        }
    }
}

