use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    account_get::{
        ra_account,
        AgoAccount,
    },
    account_post::FormData,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
    get_name_from_names::get_name_from_names,
};
use crate::database::error_codes as ec;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
};


markup::define! {
    Account<'a>(
        title: &'a str,
        q: &'a AgoAccount,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    success: success,
                    error: error,
                    form: form,
                },
                current_page: "account",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoAccount,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.account

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_account_has_been_successfully_updated
                }
            }

            @if let Some(e) = error {
                div[
                    class = "notification is-danger",
                ] {
                    button[class = "delete"] {}
                    @e.message
                }
            }

            form[
                method = "post",
                action = ra_account(&q.data.lang.code),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.email
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if (e.code == ec::WRONG_EMAIL)
                                    || (e.code == ec::EMAIL_ALREADY_EXISTS)
                                {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "email",
                            name = "email",
                            value = if let Some(f) = form {
                                &f.email
                            } else { &q.user_data.email },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.name
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_USER_NAME {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "name",
                            value = if let Some(f) = form {
                                &f.name
                            } else { &q.user_data.name },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.current_password
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_USER_PASSWORD {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "password",
                            name = "password",
                            value = "",
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.new_password

                        " ("
                        @t.optional
                        ")"
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if (e.code == ec::WRONG_PASSWORD)
                                    || (e.code == ec::PASSWORDS_DO_NOT_MATCH)
                                {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "password",
                            name = "new_password",
                            value = "",
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.repeat_new_password

                        " ("
                        @t.optional
                        ")"
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::PASSWORDS_DO_NOT_MATCH {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "password",
                            name = "repeat_password",
                            value = "",
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.names_for_each_language

                        " ("
                        @t.optional
                        ")"
                    }
                    p[class = "control"] {
                        @for (i, lang) in q.data.languages.iter().enumerate() {
                            div[class = "field has-addons is-marginless"] {
                                div[class = "control"] {
                                    span[class = "button is-static"] {
                                        @lang.name
                                    }
                                }
                                div[class = "control is-expanded"] {
                                    input[
                                        type = "hidden",
                                        name = "i18n_name_lang",
                                        value = &lang.id.to_string(),
                                    ];

                                    @if let Some(f) = form {
                                        input[
                                            class = if let Some(e) = error {
                                                if e.code ==
                                                ec::SOME_WRONG_I18N_USER_NAME
                                                {
                                                    "input is-danger"
                                                } else {
                                                    "input"
                                                }
                                            } else {
                                                "input"
                                            },
                                            type = "text",
                                            name = "i18n_name",
                                            value = &f.i18n_names[i],
                                        ];
                                    } else {
                                        input[
                                            class = if let Some(e) = error {
                                                if e.code ==
                                                ec::SOME_WRONG_I18N_USER_NAME
                                                {
                                                    "input is-danger"
                                                } else {
                                                    "input"
                                                }
                                            } else {
                                                "input"
                                            },
                                            type = "text",
                                            name = "i18n_name",
                                            value = &get_name_from_names(
                                                lang.id,
                                                &q.i18n_names,
                                            ),
                                        ];
                                    }

                                }
                            }
                        }
                    }
                }

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            @t.submit
                        }
                    }
                }
            }
        }
    }
}

