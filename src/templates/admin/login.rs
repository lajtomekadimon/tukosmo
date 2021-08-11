use markup;

use crate::templates::admin_layout::AdminLayout;


markup::define! {
    Login<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: Content {},
        }
    }

    Content() {

        section[class = "hero is-success is-fullheight"] {
            div[class = "hero-body"] {
                div[class = "container has-text-centered"] {
                    div[class = "column is-4 is-offset-4"] {
                        h3[class = "title has-text-black"] {
                            "Login"
                        }
                        hr[class = "login-hr"];
                        p[class = "subtitle has-text-black"] {
                            "Please login to proceed."
                        }
                        div[class = "box"] {
                            figure[class = "avatar"] {
                                img[src = "http://placehold.it/128x128"];  // TODO: Tukosmo logo
                            }
                            form[
                                action = "/login",
                                method = "post",
                            ] {
                                div[class = "field"] {
                                    div[class = "control"] {
                                        input[
                                            class = "input is-large",
                                            name = "email",
                                            type = "email",
                                            placeholder = "Your email",
                                            autofocus = "",
                                        ];
                                    }
                                }
                                div[class = "field"] {
                                    div[class = "control"] {
                                        input[
                                            class = "input is-large",
                                            name = "password",
                                            type = "password",
                                            placeholder = "Your password",
                                        ];
                                    }
                                }
                                div[class = "field"] {
                                    label[class = "checkbox"] {
                                        input[type = "checkbox"];
                                        " "
                                        "Remember me"
                                    }
                                }
                                button[
                                    class = "button is-block is-info is-large is-fullwidth",
                                ] {
                                    "Login"
                                    " "
                                    i[class = "eos-icons"] { "login" }
                                }
                            }
                        }

                        p[class = "has-text-grey"] {
                            a[href = "/"] {
                                "Sign up"
                            }
                            " · "

                            a[href = "/"] {
                                "Forgot password"
                            }
                            " · "
                            
                            a[href = "/"] {
                                "Need help?"
                            }
                        }
                    }
                }
            }
        }

    }
}

