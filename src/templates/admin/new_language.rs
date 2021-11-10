use actix_web::web::Form as ActixForm;
use markup;

use crate::config::global::SUPPORTED_LANGUAGES;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::get_lang_name::get_lang_name;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::new_language::NewLanguageAResponse;
use crate::handlers::admin::new_language_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    NewLanguage<'a>(
        title: &'a str,
        q: &'a NewLanguageAResponse,
        t: &'a TranslateI18N,
        auto: &'a Option<String>,
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
                    auto: auto,
                    error: error,
                    form: form,
                },
                current_page: "new_language",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a NewLanguageAResponse,
        t: &'a TranslateI18N,
        auto: &'a Option<String>,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.add_language

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        routes: &q.routes,
                        data: &q.data,
                    }
                }
            }

            p[
                class = "mb-4",
            ] {
                @t.autocomplete_for
                " "

                // TODO: Should I ignore current languages in use?
                @for (i, lang) in SUPPORTED_LANGUAGES.iter().enumerate() {
                    @if i != 0 {
                        ", "
                    }

                    a[
                        href = "/{lang}/admin/new_language?auto={code}"
                            .replace("{lang}", &q.data.lang.code)
                            .replace("{code}", lang),
                    ] {
                        {&get_lang_name(lang, &q.data.lang.code)}
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
                action = "/{lang}/admin/new_language"
                    .replace("{lang}", &q.data.lang.code),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.name_in_the_new_language
                    }
                    div[class = "control"] {
                        @if form.is_some() || auto.is_none() {
                            input[
                                class = if let Some(e) = error {
                                    if e.code == ec::WRONG_OWN_LANG_NAME {
                                        "input is-danger"
                                    } else {
                                        "input"
                                    }
                                } else {
                                    "input"
                                },
                                type = "text",
                                name = "own_lang_name",
                                placeholder = t.examples_of_lang_names,
                                value = if let Some(f) = form {
                                    &f.own_lang_name
                                } else { "" },
                            ];
                        } else if let Some(auto_code) = auto {
                            input[
                                class = "input",
                                type = "text",
                                name = "own_lang_name",
                                placeholder = t.examples_of_lang_names,
                                value = get_lang_name(auto_code, auto_code),
                            ];
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.code
                    }
                    div[class = "control"] {
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
                            } else if let Some(auto_code) = auto {
                                &auto_code
                            } else { "" },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.language_names
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
                                        name = "lang_id",
                                        value = &lang.id.to_string(),
                                    ];
                                    @if form.is_some() || auto.is_none() {
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
                                            } else { "" },
                                        ];
                                    } else if let Some(auto_code) = auto {
                                        input[
                                            class = "input",
                                            type = "text",
                                            name = "lang_name",
                                            value = &get_lang_name(
                                                auto_code,
                                                &lang.code,
                                            ),
                                        ];
                                    }
                                }
                            }
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.names_in_the_new_language_for_each_language
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
                                    @if form.is_some() || auto.is_none() {
                                        input[
                                            class = if let Some(e) = error {
                                                if e.code ==
                                                   ec::SOME_WRONG_NAME_FOR_LANG
                                                {
                                                    "input is-danger"
                                                } else {
                                                    "input"
                                                }
                                            } else {
                                                "input"
                                            },
                                            type = "text",
                                            name = "name_for_lang",
                                            value = if let Some(f) = form {
                                                &f.names_for_langs[i]
                                            } else { "" },
                                        ];
                                    } else if let Some(auto_code) = auto {
                                        input[
                                            class = "input",
                                            type = "text",
                                            name = "name_for_lang",
                                            value = &get_lang_name(
                                                &lang.code,
                                                auto_code,
                                            ),
                                        ];
                                    }
                                }
                            }
                        }
                    }
                }

                hr;

                div[class = "field"] {
                    label[class = "label"] {
                        @t.website_title_in_the_new_language
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_WEBSITE_TITLE {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "website_title",
                            value = if let Some(f) = form {
                                &f.website_title
                            } else { "" },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.website_subtitle_in_the_new_language
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_WEBSITE_SUBTITLE {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "website_subtitle",
                            value = if let Some(f) = form {
                                &f.website_subtitle
                            } else { "" },
                        ];
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
                }
            }
        }
    }
}

