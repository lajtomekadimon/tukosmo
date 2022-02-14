use markup;

use crate::handlers::admin::favicon_get::{
    AgoFavicon,
    ra_favicon,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Favicon<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoFavicon,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        success: &'a bool,
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
                    success: success,
                },
                current_page: "favicon",
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoFavicon,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        success: &'a bool,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.favicon
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
                action = ra_favicon(&q.data.lang.code),
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
                                accept = "image/png",
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

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            @t.submit
                        }
                    }
                }
            }

            hr;

            h2[class = "title is-4"] {
                @t.current_favicon
            }

            p {
                img[
                    src = "/static/favicon/favicon-96x96.png",
                    alt = "favicon-96x96.png",
                ];
            }

            p[class = "mt-3 mb-3"] {
                @t.favicon_note
            }

            hr;

            h3[class = "title is-5 mt-5"] {
                @t.web_browsers_preview
            }

            div[class = "favicon-firefox-light-preview"] {
                div[class = "favicon-firefox-tab1"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-firefox-tab2"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
            }

            div[class = "favicon-firefox-dark-preview"] {
                div[class = "favicon-firefox-tab1"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-firefox-tab2"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
            }

            div[class = "favicon-chrome-light-preview"] {
                div[class = "favicon-chrome-tab1"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-chrome-tab2"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
            }

            div[class = "favicon-chrome-dark-preview"] {
                div[class = "favicon-chrome-tab1"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-chrome-tab2"] {
                    img[
                        src = "/static/favicon/favicon-16x16.png",
                        alt = "favicon-16x16.png",
                    ];
                }
            }
        }
    }
}

