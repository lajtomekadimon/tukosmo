use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    scope_languages::edit_get::{
        AgoLanguagesEdit,
        ra_languages_edit_w_id,
    },
    scope_languages::edit_post::FormData,
    scope_languages::delete_get::ra_languages_delete_w_id,
    languages_get::{ra_languages, ra_languages_success},
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::database::error_codes as ec;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Edit<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoLanguagesEdit,
        t: &'a TranslateI18N,
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
                    error: error,
                    form: form,
                },
                current_page: "edit_language",
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoLanguagesEdit,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.edit_language_w_name
                    .replace("{name}", &q.lang.name)
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
                id = "form-langedit",
                method = "post",
                action = ra_languages_edit_w_id(
                    &q.data.lang.code,
                    &q.lang.id,
                ),
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
            }  // end form

            div[class = "field is-grouped"] {
                div[class = "control"] {
                    button[
                        id = "form-langedit-button",
                        class = "button is-link",
                        "data-nexturl" = ra_languages_success(
                            &q.data.lang.code
                        ),
                    ] {
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

                div[class = "control"] {
                    a[
                        href = ra_languages_delete_w_id(
                            &q.data.lang.code,
                            &q.lang.id,
                        ),
                        class = "button is-danger \
                                 has-text-weight-normal mr-4",
                    ] {
                        @t.delete
                    }
                }
            }

            div[
                id = "form-langedit-progress",
                class = "is-hidden",
            ] {
                progress[
                    class = "progress is-large is-link",
                    value = "80",
                    max = "100",
                ] {}
            }
        }
    }
}

