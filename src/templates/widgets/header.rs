use markup;


markup::define! {
    Header() {
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
                            href = "/",
                        ] {
                            "Lajto"
                        }
                    }

                    p[
                        class = "site-description",
                    ] {
                        "Science as a light in the dark"
                    }
                }
            }
        }
    }
}

