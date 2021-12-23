use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::website::{ra_website, WebsiteAResponse};
use crate::handlers::admin::website_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    Website<'a>(
        title: &'a str,
        q: &'a WebsiteAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
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
                    success: success,
                    error: error,
                    form: form,
                },
                current_page: "website",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a WebsiteAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
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

