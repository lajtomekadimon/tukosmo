use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::login::LoginAResponse;
use crate::handlers::admin::login_post::FormData;
use crate::database::types::{AdminDataDB, UserDB};
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    Login<'a>(
        title: &'a str,
        q: &'a LoginAResponse,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
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
                error: error,
                form: form,
            },
        }
    }

    Content<'a>(
        q: &'a LoginAResponse,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {

        section[class = "hero is-success is-fullheight"] {
            div[class = "hero-body"] {
                div[class = "container has-text-centered"] {
                    div[class = "column is-4 is-offset-4"] {
                        div[class = "box"] {
                            figure[class = "avatar"] {
                                img[src = "/static/img/tukosmo-logo-128.png"];
                            }

                            @if let Some(e) = error {
                                div[
                                    class = "notification is-danger",
                                ] {
                                    button[class = "delete"] {}
                                    @e.message
                                }
                            }

                            @Form {
                                q: q,
                                error: error,
                                form: form,
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
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        form[
            action = "/{lang}/admin/login"
                .replace("{lang}", &q.data.lang.code),
            method = "post",
        ] {
            div[class = "field"] {
                div[class = "control"] {
                    input[
                        class = if let Some(e) = error {
                            if e.code == ec::WRONG_USER_EMAIL {
                                "input is-large is-danger"
                            } else {
                                "input is-large"
                            }
                        } else {
                            "input is-large"
                        },
                        name = "email",
                        type = "email",
                        placeholder = &t("Your email", &q.data.lang.code),
                        value = if let Some(f) = form {
                            &f.email
                        } else { "" },
                        autofocus = if let Some(e) = error {
                            e.code == ec::WRONG_USER_EMAIL
                        } else {
                            true
                        },
                    ];
                }
            }

            div[class = "field"] {
                div[class = "control"] {
                    input[
                        class = if let Some(e) = error {
                            if e.code == ec::WRONG_USER_PASSWORD {
                                "input is-large is-danger"
                            } else {
                                "input is-large"
                            }
                        } else {
                            "input is-large"
                        },
                        name = "password",
                        type = "password",
                        placeholder = &t("Your password", &q.data.lang.code),
                        autofocus = if let Some(e) = error {
                            e.code == ec::WRONG_USER_PASSWORD
                        } else {
                            false
                        },
                    ];
                }
            }

            button[
                class = "button is-block is-link is-large is-fullwidth",
            ] {
                {&t("Login [verb]", &q.data.lang.code)}
                " "
                i[class = "eos-icons"] { "login" }
            }
        }
    }
}

