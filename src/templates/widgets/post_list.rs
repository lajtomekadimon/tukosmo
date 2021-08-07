use markup;


markup::define! {
    PostList() {
        div[
            class = "post-list",
        ] {
            @PostWrapper {}
            @PostWrapper {}
            @PostWrapper {}
        }
    }

    PostWrapper() {
        section[
            class = "post-wrapper"
        ] {
            div[
                class = "post-wrapper-image"
            ] {
                a[
                    href = "/",
                ] {
                    figure[
                        style = "background-image: url(https://www.azamara.com/sites/default/files/heros/reykjavik-iceland-1800x1000.jpg);",
                    ] {}
                }
            }

            div[
                class = "post-wrapper-data",
            ] {
                div[
                    class = "post-wrapper-data-meta",
                ] {
                    div[
                        class = "post-wrapper-data-meta-date",
                    ] {
                        a[
                            href = "/",
                        ] {
                            time[
                                datetime = "2021-08-11T20:37:29+00:00",
                            ] {
                                "August 1, 2021"
                            }
                        }
                    }
                }

                h2 {
                    a[
                        href = "/",
                    ] {
                        "The Reykjavík test post"
                    }
                }

                p {
                    "Bla bla bla bla bla bla bla bla bla "
                    "Bla bla bla bla bla bla bla bla bla "
                    "Bla bla bla bla bla bla bla bla bla "
                    "Bla bla bla bla bla bla bla bla bla "
                    "Bla bla bla bla bla bla bla bla bla "
                    "Bla bla bla bla bla bla bla bla bla."
                }

                div[
                    class = "post-wrapper-data-more",
                ] {
                    a[
                        href = "/",
                    ] {
                        "Read more"
                    }
                }
            }
        }
    }
}

