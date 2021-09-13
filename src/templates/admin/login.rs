use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::database::data::CurrentLanguageDB;


markup::define! {
    Login<'a>(
        title: &'a str,
        lang: &'a CurrentLanguageDB,
    ) {
        @AdminLayout {
            title: title,
            lang: lang,
            content: Content {
                lang: lang,
            },
        }
    }

    Content<'a>(
        lang: &'a CurrentLanguageDB,
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
                                action = "/{lang}/admin/login"
                                    .replace("{lang}", &lang.code),
                                method = "post",
                            ] {
                                div[class = "field"] {
                                    div[class = "control"] {
                                        input[
                                            class = "input is-large",
                                            name = "email",
                                            type = "email",
                                            placeholder = &t("Your email", &lang.code),
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
                                            placeholder = &t("Your password", &lang.code),
                                        ];
                                    }
                                }

                                button[
                                    class = "button is-block is-info is-large is-fullwidth",
                                ] {
                                    {&t("Login [verb]", &lang.code)}
                                    " "
                                    i[class = "eos-icons"] { "login" }
                                }
                            }
                        }

                        p[class = "has-text-grey"] {
                            a[href = "/"] {
                                {&t("Sign up [verb]", &lang.code)}
                            }

                            " · "

                            a[href = "/"] {
                                {&t("Forgotten password?", &lang.code)}
                            }
                        }
                    }
                }
            }
        }

    }
}

