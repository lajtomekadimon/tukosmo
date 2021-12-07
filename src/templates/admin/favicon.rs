use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::favicon::FaviconAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    Favicon<'a>(
        title: &'a str,
        q: &'a FaviconAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        success: &'a bool,
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
                    success: success,
                },
                current_page: "favicon",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a FaviconAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.favicon

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

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_favicon_was_successfully_updated
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/favicon"
                    .replace("{lang}", &q.data.lang.code)
                ,
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
                        @t.upload_new_favicon

                        " ("
                        @t.png_image
                        ")"
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
                                    "Choose a file..."
                                }
                            }
                            span[class = "file-name"] {
                                "No file uploaded"
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

            hr;

            h2[class="title is-4"] {
                @t.current_favicon
            }

            p {
                img[
                    src = "/static/favicon/favicon-96x96.png",
                    alt = "favicon-96x96.png",
                ];
            }
        }
    }
}

