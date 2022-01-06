use markup;

use crate::files::static_files;
use crate::database::types::{AdminDataDB, RouteDB};
use crate::templates::widgets::faviconadmin_meta::FaviconAdminMeta;


markup::define! {
    AdminLayout<'a, BodyContent: markup::Render>(
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
                @FaviconAdminMeta {}

                // i18n routes
                @for route in routes.iter() {
                    link[
                        rel = "alternate",
                        hreflang = &route.lang.code,
                        href = "https://tukosmo.org{route}"  // TODO: Domain!!
                            .replace("{route}", &route.route),
                    ];
                }

                link[
                    rel = "stylesheet",
                    href = "https://cdn.jsdelivr.net/npm/\
                            eos-icons@latest/dist/css/eos-icons.css",
                ];

                link[
                    rel = "stylesheet",
                    href = static_files::CSS_ADMIN,
                ];
            }
            body {
                @content

                // JavaScript
                script[src = static_files::JS_ADMIN] {}
            }
        }
    }

}

