use markup;


markup::define! {
    AdminNavbar() {
        nav[class = "navbar is-white"] {
            div[class = "container"] {
                div[class = "navbar-brand"] {
                    a[
                        class = "navbar-item brand-text",
                        href = "/",
                    ] {
                        "Tukosmo Admin Panel"
                    }
                    div[class = "navbar-burger burger"] {
                        span {}
                        span {}
                        span {}
                    }
                }
                div[
                    id = "navMenu",
                    class = "navbar-menu",
                ] {
                    div[class = "navbar-start"] {
                        a[
                            class = "navbar-item",
                            href = "/",
                        ] {
                            "Visit website"
                        }
                        a[
                            class = "navbar-item",
                            href = "/",
                        ] {
                            "Documentation"
                        }
                        a[
                            class = "navbar-item",
                            href = "/",
                        ] {
                            "Help"
                        }
                    }
                }
            }
        }
    }
}

