use actix_web::web::Form as ActixForm;
use markup;

use crate::config::global::Config;
use crate::config::global::SUPPORTED_LANGUAGES;
use crate::handlers::admin::{
    scope_languages::new_get::{
        AgoLanguagesNew,
        ra_languages_new,
        ra_languages_new_w_auto,
    },
    scope_languages::new_post::FormData,
    languages_get::ra_languages,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    get_lang_name::get_lang_name,
    t_error::ErrorDB,
};
use crate::database::error_codes as ec;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    New<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
        title: &'a str,
        q: &'a AgoLanguagesNew,
        t: &'a TranslateI18N,
        auto: &'a Option<String>,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
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
                codename: codename,
                config: config,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoLanguagesNew,
        t: &'a TranslateI18N,
        auto: &'a Option<String>,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.add_language
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
                        href = ra_languages_new_w_auto(
                            &q.data.lang.code,
                            lang,
                        ),
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
                action = ra_languages_new(&q.data.lang.code),
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

                div[class = "field"] {
                    label[class = "label"] {
                        @t.copyright_owner_in_the_new_language
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_COPYRIGHT_OWNER {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "copyright_owner",
                            value = if let Some(f) = form {
                                &f.copyright_owner
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
                            href = ra_languages(&q.data.lang.code),
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

