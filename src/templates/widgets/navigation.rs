use markup;


markup::define! {
    Navigation() {
        nav[
            class = "site-navigation",
        ] {
            div [
                class = "container",
            ] {
                ul[
                    class = "site-navigation-ul",
                ] {
                    li[
                        class = "site-navigation-li",
                    ] {
                        a[
                            href = "/",
                        ] {
                            "Blog"
                        }
                    }

                    li[
                        class = "site-navigation-li",
                    ] {
                        a[
                            href = "/",
                        ] {
                            "About me"
                        }
                    }

                    li[
                        class = "site-navigation-li",
                    ] {
                        a[
                            href = "/",
                        ] {
                            "Contact"
                        }
                    }
                }
            }
        }
    }
}

