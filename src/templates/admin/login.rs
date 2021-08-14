use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;


markup::define! {
    Login<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: Content {
                lang_code: lang_code
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
    ) {

        section[class = "hero is-success is-fullheight"] {
            div[class = "hero-body"] {
                div[class = "container has-text-centered"] {
                    div[class = "column is-4 is-offset-4"] {
                        div[class = "box"] {
                            figure[class = "avatar"] {
                                img[src = "/static/img/tukosmo-logo-128.png"];
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
                                            placeholder = &t("Your email", lang_code),
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
                                            placeholder = &t("Your password", lang_code),
                                        ];
                                    }
                                }
                                button[
                                    class = "button is-block is-info is-large is-fullwidth",
                                ] {
                                    {&t("Login [verb]", lang_code)}
                                    " "
                                    i[class = "eos-icons"] { "login" }
                                }
                            }
                        }

                        p[class = "has-text-grey"] {
                            a[href = "/"] {
                                {&t("Sign up [verb]", lang_code)}
                            }

                            " · "

                            a[href = "/"] {
                                {&t("Forgotten password?", lang_code)}
                            }
                        }
                    }
                }
            }
        }

    }
}

