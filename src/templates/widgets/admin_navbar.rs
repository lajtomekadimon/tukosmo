use markup;


markup::define! {
    AdminNavbar() {
        nav[class = "navbar is-white"] {
            div[class = "container"] {
                div[class = "navbar-brand"] {
                    a[
                        class = "navbar-item brand-text",
                        href = "/en/admin/",  // TODO
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
                            href = "/en/",  // TODO
                            target = "_blank",
                        ] {
                            "Visit website"
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's docs
                            target = "_blank",
                        ] {
                            "Documentation"
                        }
                        a[
                            class = "navbar-item",
                            href = "/",  // link to official Tukosmo's help
                            target = "_blank",
                        ] {
                            "Help"
                        }
                    }

                    div[class = "navbar-end"] {
                        div[class = "navbar-item has-dropdown is-hoverable"] {
                            a[class = "navbar-link"] {
                                "Lajto (test@test.com)"
                            }

                            div[class = "navbar-dropdown is-right"] {
                                a[class = "navbar-item"] {
                                    "Profile"
                                }
                                a[class = "navbar-item"] {
                                    "Sessions"
                                }
                                a[class = "navbar-item"] {
                                    "Settings"
                                }

                                hr[class = "navbar-divider"];

                                a[
                                    href = "/logout",
                                    class = "navbar-item",
                                ] {
                                    "Logout"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

