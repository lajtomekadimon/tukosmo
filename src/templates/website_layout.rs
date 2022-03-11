use markup;

use crate::files::static_files::{
    staticf_route,
    CSS_WEBSITE,
    JS_WEBSITE,
};
use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{WebsiteDataDB, RouteDB};
use crate::handlers::website::scope_rss::blog_get::rw_rss_blog;
use crate::templates::widgets::{
    favicon_meta::FaviconMeta,
    open_graph_meta::{OpenGraphMeta, ArticleOG},
};


markup::define! {
    WebsiteLayout<'a, BodyContent: markup::Render>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
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
                @FaviconMeta {
                    codename: codename,
                }

                // RSS
                link[
                    rel = "alternate",
                    type = "application/rss+xml",
                    title = "{web} - {blog}"
                        .replace("{web}", &data.website_title)
                        .replace("{blog}", &t.blog),
                    href = "https://{domain}{route}"
                        .replace("{domain}", domain)
                        .replace("{route}", &rw_rss_blog(
                            &data.lang.code
                        )),
                ];

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

                // CSS
                /*link[
                    rel = "stylesheet",
                    href = "https://fonts.googleapis.com/css?family=Poppins",
                ];*/
                link[
                    rel = "stylesheet",
                    type = "text/css",
                    href = staticf_route(CSS_WEBSITE, codename),
                ];
            }
            body {
                @content

                // JavaScript
                script[
                    src = staticf_route(JS_WEBSITE, codename),
                ] {}
            }
        }
    }
}

