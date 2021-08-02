use markup;


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
                link[
                    rel = "apple-touch-icon",
                    sizes = "",
                    href = "",
                ];

                // These are many
                link[
                    rel = "icon",
                    type = "",
                    sizes = "",
                    href = "",
                ];

                link[
                    rel = "manifest",
                    href = "",
                ];

                // Styles
                link[
                    rel = "stylesheet",
                    href = "/static/bundle.css",
                ];
            }
            body {
                @content

                script[
                    src = "",
                ] {}
            }
        }
    }
}

