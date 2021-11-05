use markup;

use crate::database::types::WebsiteDataDB;


markup::define! {
    Header<'a>(
        data: &'a WebsiteDataDB,
    ) {
        header[
            class = "site-header",
        ] {
            div[
                class = "container",
            ] {
                div[
                    class = "site-branding",
                ] {
                    h1[
                        class = "site-title",
                    ] {
                        a[
                            href = "/{lang}/"
                                .replace("{lang}", &data.lang.code),
                        ] {
                            @data.website_title
                        }
                    }

                    p[
                        class = "site-description",
                    ] {
                        @data.website_subtitle
                    }
                }
            }
        }
    }
}

