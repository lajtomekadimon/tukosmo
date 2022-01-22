use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    website_get::{ra_website, AgoWebsite},
    website_post::FormData,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::database::error_codes as ec;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
};


markup::define! {
    Website<'a>(
        domain: &'a str,
        title: &'a str,
        q: &'a AgoWebsite,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        default_lang: &'a str,
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
                    success: success,
                    error: error,
                    form: form,
                    domain: domain,
                    default_lang: default_lang,
                },
                current_page: "website",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoWebsite,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
        domain: &'a str,
        default_lang: &'a str,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.website

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_info_has_been_successfully_updated
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
                action = ra_website(&q.data.lang.code),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.website_title
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
                            } else { &q.website_title },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.website_subtitle
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
                            } else { &q.website_subtitle },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.copyright_owner
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
                            } else { &q.copyright_owner },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.domain_k_web
                        // TODO: Add "DANGEROUS" and a message or something
                        // to indicate that you must have the domain pointing
                        // to the server's IP, etc.
                    }
                    div[class = "control"] {
                        input[
                            class = "input",  // TODO: Check
                            type = "text",
                            name = "domain",
                            value = if let Some(f) = form {
                                &f.domain
                            } else { *domain },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.default_language
                    }
                    div[class = "control"] {
                        div[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_LANG_CODE {
                                    "select is-danger"
                                } else {
                                    "select"
                                }
                            } else {
                                "select"
                            },
                        ] {
                            select[
                                name = "default_lang",
                            ] {
                                @for lang in q.data.languages.iter() {
                                    option[
                                        value = &lang.code,
                                        selected = if let Some(f) = form {
                                            lang.code == f.default_lang
                                        } else {
                                            &lang.code == default_lang
                                        },
                                    ] {
                                        @lang.name

                                        @if q.data.lang.code != lang.code {
                                            " ("
                                            @lang.original_name
                                            ")"
                                        }
                                    }
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
                }
            }
        }
    }
}

