use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::login::LoginAResponse;
use crate::database::types::{AdminDataDB, UserDB};


markup::define! {
    Login<'a>(
        title: &'a str,
        q: &'a LoginAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &AdminDataDB {
                userd: UserDB {
                    id: 0,
                    email: "".to_string(),
                    name: "".to_string(),
                },
                lang: q.data.lang.clone(),
                languages: q.data.languages.clone(),
            },
            content: Content {
                q: q,
            },
        }
    }

    Content<'a>(
        q: &'a LoginAResponse,
    ) {

        section[class = "hero is-success is-fullheight"] {
            div[class = "hero-body"] {
                div[class = "container has-text-centered"] {
                    div[class = "column is-4 is-offset-4"] {
                        div[class = "box"] {
                            figure[class = "avatar"] {
                                img[src = "/static/img/tukosmo-logo-128.png"];
                            }

                            @Form {
                                q: q,
                            }
                        }

                        p[class = "has-text-grey"] {
                            a[href = "/"] {
                                {&t("Sign up [verb]", &q.data.lang.code)}
                            }

                            " · "

                            a[href = "/"] {
                                {&t("Forgotten password?", &q.data.lang.code)}
                            }
                        }

                        div[class = "mt-3"] {
                            @AdminLangDropdown {
                                route: "/admin/login",
                                data: &AdminDataDB {
                                    userd: UserDB {
                                        id: 0,
                                        email: "".to_string(),
                                        name: "".to_string(),
                                    },
                                    lang: q.data.lang.clone(),
                                    languages: q.data.languages.clone(),
                                },
                            }
                        }
                    }
                }
            }
        }

    }

    Form<'a>(
        q: &'a LoginAResponse,
    ) {
        form[
            action = "/{lang}/admin/login"
                .replace("{lang}", &q.data.lang.code),
            method = "post",
        ] {
            div[class = "field"] {
                div[class = "control"] {
                    input[
                        class = "input is-large",
                        name = "email",
                        type = "email",
                        placeholder = &t("Your email", &q.data.lang.code),
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
                        placeholder = &t("Your password", &q.data.lang.code),
                    ];
                }
            }

            button[
                class = "button is-block is-info is-large is-fullwidth",
            ] {
                {&t("Login [verb]", &q.data.lang.code)}
                " "
                i[class = "eos-icons"] { "login" }
            }
        }
    }
}

