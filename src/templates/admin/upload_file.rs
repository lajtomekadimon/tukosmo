use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::upload_file::UploadFileAResponse;
use crate::i18n::t_error::ErrorDB;
use crate::handlers::admin::upload_file::ra_upload_file;
use crate::handlers::admin::files::ra_files;


markup::define! {
    UploadFile<'a>(
        title: &'a str,
        q: &'a UploadFileAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
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
                },
                current_page: "upload_file",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a UploadFileAResponse,
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

            form[
                method = "post",
                action = ra_upload_file(&q.data.lang.code),
                enctype = "multipart/form-data",
            ] {
                // Sadly, Actix doesn't support this in multipart yet
                /*
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];
                */

                div[class = "field"] {
                    label[class = "label"] {
                        @t.file
                    }
                    div[
                        id = "file-js",
                        class = "file has-name",
                    ] {
                        label[class = "file-label"] {
                            input[
                                class = "file-input",
                                type = "file",
                                name = "resume",
                            ];
                            span[class = "file-cta"] {
                                span[class = "file-icon"] {
                                    i[class = "eos-icons"] { "cloud_upload" }
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

