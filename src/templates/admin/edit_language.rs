use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_language::EditLanguageAResponse;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    EditLanguage<'a>(
        title: &'a str,
        q: &'a EditLanguageAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    error: error,
                },
                current_page: "edit_language",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a EditLanguageAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit language: {name}",
                    &q.data.lang.code
                ).replace("{name}", &q.lang.name)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/edit_language?id={id}"
                            .replace("{id}", &q.lang.id.to_string()),
                        data: &q.data,
                    }
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
                action = "/{lang}/admin/edit_language?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &q.lang.id.to_string()),
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Code", &q.data.lang.code)}
                    }
                    div[class = "control"] {
                        input[
                            type = "hidden",
                            name = "language_id",
                            value = &q.lang.id.to_string(),
                        ];
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::FIELD_IS_NOT_LANG_CODE ||
                                    e.code == ec::LANG_CODE_ALREADY_EXISTS
                                {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "lang_code",
                            value = &q.lang.code,
                            placeholder = &t("Example: en", &q.data.lang.code),
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Language names", &q.data.lang.code)}
                    }
                    p[class = "control"] {
                        @for name in q.names.iter() {
                            div[class = "field has-addons is-marginless"] {
                                div[class = "control"] {
                                    span[class = "button is-static"] {
                                        @name.lang.name
                                    }
                                }
                                div[class = "control is-expanded"] {
                                    input[
                                        type = "hidden",
                                        name = "lang_id",
                                        value = &name.lang.id.to_string(),
                                    ];
                                    input[
                                        class = if let Some(e) = error {
                                            if e.code ==
                                                ec::SOME_WRONG_LANG_NAME
                                            {
                                                "input is-danger"
                                            } else {
                                                "input"
                                            }
                                        } else {
                                            "input"
                                        },
                                        type = "text",
                                        name = "lang_name",
                                        value = &name.name,
                                    ];
                                }
                            }
                        }
                    }
                }

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            {&t("Submit", &q.data.lang.code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/languages"
                                .replace("{lang}", &q.data.lang.code),
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

