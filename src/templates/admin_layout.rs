use markup;

use crate::files::static_files::{
    staticf_route,
    CSS_ADMIN,
    JS_ADMIN,
};
use crate::database::types::{AdminDataDB, RouteDB};
use crate::templates::widgets::faviconadmin_meta::FaviconAdminMeta;


markup::define! {
    AdminLayout<'a, BodyContent: markup::Render>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        data: &'a AdminDataDB,
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

                /*
                meta[
                    name = "description",
                    content = "",
                ];
                meta[
                    name = "author",
                    content = "",
                ];
                */

                // Favicon
                @FaviconAdminMeta {
                    codename: codename,
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
                link[
                    rel = "stylesheet",
                    type = "text/css",
                    href = staticf_route(CSS_ADMIN, codename),
                ];
            }
            body {
                @content

                // JavaScript
                script[
                    src = staticf_route(JS_ADMIN, codename),
                ] {}
            }
        }
    }

}

