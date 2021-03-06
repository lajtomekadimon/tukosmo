use markup;

use crate::config::global::Config;
use crate::files::static_files::{
    staticf_route,
    FAVICON_96X96,
    FAVICON_16X16,
    FIREFOX_TABS_LIGHT,
    FIREFOX_TABS_DARK,
    CHROME_TABS_LIGHT,
    CHROME_TABS_DARK,
};
use crate::handlers::admin::favicon_get::{
    AgoFavicon,
    ra_favicon,
    ra_favicon_success,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::{
        admin_panel::AdminPanel,
        icons,
    },
};


markup::define! {
    Favicon<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
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
                    codename: codename,
                },
                current_page: "favicon",
                codename: codename,
                config: config,
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
        codename: &'a str,
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

            div[
                class = "notification is-info",
            ] {
                button[class = "delete"] {}
                @t.tukosmo_will_automatically_restart_itself_apply_changes
            }

            form[
                id = "form-favicon",
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
                        @t.png_image_of_at_least_192x192
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
                                    @icons::CloudUpload {}
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
            }  // end form

            div[class = "field is-grouped"] {
                div[class = "control"] {
                    button[
                        id = "form-favicon-button",
                        class = "button is-link",
                        "data-nexturl" = ra_favicon_success(
                            &q.data.lang.code
                        ),
                    ] {
                        @t.submit
                    }
                }
            }

            div[
                id = "form-favicon-progress",
                class = "is-hidden",
            ] {
                progress[
                    class = "progress is-large is-link",
                    value = "80",
                    max = "100",
                ] {}
            }

            hr;

            h2[class = "title is-4"] {
                @t.current_favicon
            }

            p {
                img[
                    src = staticf_route(FAVICON_96X96, codename),
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

            div[
                class = "favicon-firefox-preview",
                style = "background-image:url({url})".replace(
                    "{url}",
                    &staticf_route(FIREFOX_TABS_LIGHT, codename),
                ),
            ] {
                div[class = "favicon-firefox-tab1"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-firefox-tab2"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
            }

            div[
                class = "favicon-firefox-preview",
                style = "background-image:url({url})".replace(
                    "{url}",
                    &staticf_route(FIREFOX_TABS_DARK, codename),
                ),
            ] {
                div[class = "favicon-firefox-tab1"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-firefox-tab2"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
            }

            div[
                class = "favicon-chrome-preview",
                style = "background-image:url({url})".replace(
                    "{url}",
                    &staticf_route(CHROME_TABS_LIGHT, codename),
                ),
            ] {
                div[class = "favicon-chrome-tab1"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-chrome-tab2"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
            }

            div[
                class = "favicon-chrome-preview",
                style = "background-image:url({url})".replace(
                    "{url}",
                    &staticf_route(CHROME_TABS_DARK, codename),
                ),
            ] {
                div[class = "favicon-chrome-tab1"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
                div[class = "favicon-chrome-tab2"] {
                    img[
                        src = staticf_route(FAVICON_16X16, codename),
                        alt = "favicon-16x16.png",
                    ];
                }
            }
        }
    }
}

