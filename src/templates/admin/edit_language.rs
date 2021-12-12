use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_language::EditLanguageAResponse;
use crate::handlers::admin::edit_language_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    EditLanguage<'a>(
        title: &'a str,
        q: &'a EditLanguageAResponse,
        t: &'a TranslateI18N,
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
                    error: error,
                    form: form,
                },
                current_page: "edit_language",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a EditLanguageAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.edit_language_w_name
                    .replace("{name}", &q.lang.name)

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
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
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.code
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
                            placeholder = t.examples_of_lang_codes,
                            value = if let Some(f) = form {
                                &f.lang_code
                            } else { &q.lang.code },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.language_names
                    }
                    p[class = "control"] {
                        @for (i, name) in q.names.iter().enumerate() {
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
                                        value = if let Some(f) = form {
                                            &f.lang_names[i]
                                        } else { &name.name },
                                    ];
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
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/languages"
                                .replace("{lang}", &q.data.lang.code),
                            class = "button is-link is-light",
                        ] {
                            @t.cancel
                        }
                    }

                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/delete_language?id={id}"
                                .replace("{lang}", &q.data.lang.code)
                                .replace("{id}", &q.lang.id.to_string()),
                            class = "button is-danger \
                                     has-text-weight-normal mr-4",
                        ] {
                            @t.delete
                        }
                    }
                }
            }
        }
    }
}

