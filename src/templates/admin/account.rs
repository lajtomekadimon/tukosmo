use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::account::AccountAResponse;
use crate::handlers::admin::account_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;
use crate::i18n::get_name_from_names::get_name_from_names;


markup::define! {
    Account<'a>(
        title: &'a str,
        q: &'a AccountAResponse,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    success: success,
                    error: error,
                    form: form,
                },
                current_page: "account",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a AccountAResponse,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Account", &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/account",
                        data: &q.data,
                    }
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    {&t(
                        "Your account has been successfully updated.",
                        &q.data.lang.code
                    )}
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
                action = "/{lang}/admin/account"
                    .replace("{lang}", &q.data.lang.code),
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Email", &q.data.lang.code)}
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
                        {&t("Name", &q.data.lang.code)}
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
                        {&t("Current password", &q.data.lang.code)}
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
                        {&t("New password", &q.data.lang.code)}

                        " ("
                        {&t("optional", &q.data.lang.code)}
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
                        {&t("Repeat new password", &q.data.lang.code)}

                        " ("
                        {&t("optional", &q.data.lang.code)}
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
                        {&t("Names for each language", &q.data.lang.code)}

                        " ("
                        {&t("optional", &q.data.lang.code)}
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

                // TODO
                /*
                div[class = "field"] {
                    div[class = "control"] {
                        label[class = "checkbox"] {
                            input[
                                type = "checkbox",
                                name = "is_admin",
                                value = "yes",
                                checked = if let Some(f) = form {
                                    match &f.is_admin {
                                        Some(_) => true,
                                        None => false,
                                    }
                                } else { false },
                            ];
                            " "
                            {&t("Admin", &q.data.lang.code)}
                        }
                    }
                }
                */

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            {&t("Submit", &q.data.lang.code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/users"
                                .replace("{lang}", &q.data.lang.code)
                            ,
                            class = "button is-link is-light",
                        ] {
                            {&t("Cancel", &q.data.lang.code)}
                        }
                    }
                }
            }
        }
    }
}

