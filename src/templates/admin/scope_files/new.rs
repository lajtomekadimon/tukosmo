use markup;

use crate::handlers::admin::{
    scope_files::new_get::{
        AgoFilesNew,
        ra_files_new,
    },
    files_get::ra_files,
    scope_json::upload_file_post::ra_json_upload_file,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
};


markup::define! {
    New<'a>(
        domain: &'a str,
        title: &'a str,
        q: &'a AgoFilesNew,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            domain: domain,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    error: error,
                },
                current_page: "upload_file",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoFilesNew,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.upload_file

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

            // This should be out of form, because it's sent using JavaScript
            div[class = "field"] {
                label[class = "label"] {
                    @t.file
                }
                div[
                    id = "file-upload",
                    class = "file has-name",
                ] {
                    label[class = "file-label"] {
                        input[
                            id = "file-upload-file",
                            "data-url" = ra_json_upload_file(&q.data.lang.code),
                            class = "file-input",
                            type = "file",
                            name = "resume",
                        ];
                        span[class = "file-cta"] {
                            span[class = "file-icon"] {
                                i[
                                    class = "eos-icons notranslate",
                                    translate = "no",
                                ] { "cloud_upload" }
                            }
                            span[class = "file-label"] {
                                @t.choose_a_file
                            }
                        }
                        span[class = "file-name"] {
                            @t.no_file_uploaded
                        }
                    }
                }
            }

            progress[
                id = "file-upload-progress",
                class = "progress is-large is-info",
                style = "display: none;",
            ] {}

            div[
                id = "file-upload-notif-success",
                class = "notification is-success",
                style = "display: none;",
            ] {
                button[class = "delete"] {}
                @t.your_file_has_been_successfully_uploaded
            }

            form[
                id = "file-upload-form",
                method = "post",
                action = ra_files_new(&q.data.lang.code),
                style = "display: none;",
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                input[
                    type = "hidden",
                    name = "id",
                    value = "",
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.change_filename_question
                    }
                    div[class = "control"] {
                        input[
                            class = "input",
                            type = "text",
                            name = "filename",
                            value = "",
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
                            href = ra_files(&q.data.lang.code),
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

