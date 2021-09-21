use markup;

use crate::database::types::LanguageDB;


markup::define! {
    AdminLayout<'a, BodyContent: markup::Render>(
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

                link[
                    rel = "apple-touch-icon",
                    sizes = "57x57",
                    href = "/static/faviconadmin/apple-icon-57x57.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "60x60",
                    href = "/static/faviconadmin/apple-icon-60x60.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "72x72",
                    href = "/static/faviconadmin/apple-icon-72x72.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "76x76",
                    href = "/static/faviconadmin/apple-icon-76x76.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "114x114",
                    href = "/static/faviconadmin/apple-icon-114x114.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "120x120",
                    href = "/static/faviconadmin/apple-icon-120x120.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "144x144",
                    href = "/static/faviconadmin/apple-icon-144x144.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "152x152",
                    href = "/static/faviconadmin/apple-icon-152x152.png",
                ];
                link[
                    rel = "apple-touch-icon",
                    sizes = "180x180",
                    href = "/static/faviconadmin/apple-icon-180x180.png",
                ];

                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "32x32",
                    href = "/static/faviconadmin/favicon-32x32.png",
                ];
                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "96x96",
                    href = "/static/faviconadmin/favicon-96x96.png",
                ];
                link[
                    rel = "icon",
                    type = "image/png",
                    sizes = "16x16",
                    href = "/static/faviconadmin/favicon-16x16.png",
                ];

                link[
                    rel = "manifest",
                    href = "/static/faviconadmin/manifest.json",
                ];

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
                @content

                /*
                script[
                    src = "",
                ] {}
                */
            }
        }
    }

}

