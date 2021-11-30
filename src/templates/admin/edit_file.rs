use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_file::EditFileAResponse;
use crate::handlers::admin::edit_file_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;
use crate::files::extensions::IMG_EXTS;


markup::define! {
    EditFile<'a>(
        title: &'a str,
        q: &'a EditFileAResponse,
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
                current_page: "edit_file",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a EditFileAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.edit_file

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        routes: &q.routes,
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
                action = "/{lang}/admin/edit_file?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &q.file_data.id.to_string()),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                input[
                    type = "hidden",
                    name = "id",
                    value = &q.file_data.id,
                ];

                div[class = "columns"] {

                    // File card
                    div[class = "column"] {
                        div[class = "card"] {
                            div[class = "card-content"] {
                                @if IMG_EXTS.contains(
                                    &q.file_data.ext.as_str(),
                                ) {
                                    img[
                                        src = "/files/{name}".replace(
                                            "{name}",
                                            &q.file_data.name,
                                        ),
                                    ];
                                }

                                // TODO: File size

                                div {
                                    strong {
                                        @t.uploaded_on
                                        ": "
                                    }
                                    {&t_date(&q.file_data.date, &q.data.lang.code)}
                                }

                                div {
                                    strong {
                                        @t.uploaded_by
                                        ": "
                                    }

                                    a[
                                        href = "",
                                    ] {
                                        "User name"
                                    }
                                }
                            }
                            div[class = "card-footer"] {
                                div[class = "card-footer-item"] {
                                    span {
                                        a[
                                            href = "/files/{name}".replace(
                                                "{name}",
                                                &q.file_data.name,
                                            ),
                                            target = "_blank",
                                        ] {
                                            "/files/"
                                            @q.file_data.name
                                        }
                                    }
                                }
                                div[class = "card-footer-item"] {
                                    span {
                                        a[
                                            class = "button is-link is-light",
                                            href = "/files/{name}".replace(
                                                "{name}",
                                                &q.file_data.name,
                                            ),
                                            download = true,
                                        ] {
                                            @t.download
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Edit file
                    div[class = "column"] {
                        div[class = "field"] {
                            label[class = "label"] {
                                @t.filename
                            }
                            div[class = "control"] {
                                input[
                                    class = if let Some(e) = error {
                                        if e.code == ec::WRONG_FILENAME {
                                            "input is-danger"
                                        } else {
                                            "input"
                                        }
                                    } else {
                                        "input"
                                    },
                                    type = "text",
                                    name = "filename",
                                    value = if let Some(f) = form {
                                        &f.filename
                                    } else { &q.file_data.name },
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
                                    href = "/{lang}/admin/files"
                                        .replace("{lang}", &q.data.lang.code)
                                    ,
                                    class = "button is-link is-light",
                                ] {
                                    @t.cancel
                                }
                            }

                            div[class = "control"] {
                                a[
                                    href = "/{lang}/admin/delete_file?id={id}"
                                        .replace("{lang}", &q.data.lang.code)
                                        .replace(
                                            "{id}",
                                            &q.file_data.id.to_string(),
                                        ),
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
    }
}

