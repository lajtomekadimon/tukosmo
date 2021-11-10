use markup;

use crate::database::types::WebsiteDataDB;


markup::define! {
    Navigation<'a>(
        data: &'a WebsiteDataDB,
    ) {
        nav[
            class = "site-navigation",
        ] {
            div [
                class = "container",
            ] {
                ul[
                    class = "site-navigation-ul",
                ] {
                    /*
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
                    */

                    li[
                        class = "site-navigation-li",
                    ] {
                        button[
                            id = "button-select-language",
                        ] {
                            i[class = "eos-icons eos-icons-l"] { "language" }
                            {data.lang.code.to_uppercase()}
                            i[class = "eos-icons eos-icons-r"] { "arrow_drop_down" }
                        }
                    }
                }
            }
        }
    }
}

