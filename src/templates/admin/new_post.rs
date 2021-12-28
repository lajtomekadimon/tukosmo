use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::new_post::NewPostAResponse;
use crate::handlers::admin::new_post_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;
use crate::handlers::admin::new_post::ra_new_post;
use crate::handlers::admin::posts::ra_posts;
use crate::templates::widgets::admin_post_editor::AdminPostEditor;
use crate::templates::widgets::admin_file_selector::AdminFileSelector;


markup::define! {
    NewPost<'a>(
        title: &'a str,
        q: &'a NewPostAResponse,
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
                current_page: "new_post",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a NewPostAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.new_post

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

            // File selector!!!
            div[
                id = "file-selector-panel",
                class = "modal",
            ] {}

            form[
                method = "post",
                action = ra_new_post(&q.data.lang.code),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        @t.title
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_TITLE {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "title",
                            value = if let Some(f) = form {
                                &f.title
                            } else { "" },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.featured_image
                    }
                    div[class = "control"] {
                        @AdminFileSelector {
                            t: t,
                            name: "featured_image",
                            current_file: if let Some(_f) = form {
                                &q.featured_image
                            } else { &None },
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.permalink
                    }
                    div[class = "control"] {
                        input[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_PERMALINK {
                                    "input is-danger"
                                } else {
                                    "input"
                                }
                            } else {
                                "input"
                            },
                            type = "text",
                            name = "permalink",
                            value = if let Some(f) = form {
                                &f.permalink
                            } else { "" },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.description
                    }
                    div[class = "control"] {
                        textarea[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_DESCRIPTION {
                                    "textarea has-fixed-size is-danger"
                                } else {
                                    "textarea has-fixed-size"
                                }
                            } else {
                                "textarea has-fixed-size"
                            },
                            name = "description",
                            rows = "3",
                        ] {
                            @if let Some(f) = form {
                                {&f.description}
                            } else { "" }
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.post_s_body
                    }
                    div[class = "control"] {
                        @AdminPostEditor {
                            name: "body",
                            init_value: if let Some(f) = form {
                                &f.body
                            } else { "" },
                        }
                    }
                }

                div[class = "field"] {
                    div[class = "control"] {
                        label[class = "checkbox"] {
                            input[
                                type = "checkbox",
                                name = "draft",
                                value = "yes",
                                checked = if let Some(f) = form {
                                    match &f.draft {
                                        Some(_) => true,
                                        None => false,
                                    }
                                } else { false },
                            ];
                            " "
                            @t.draft
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
                            href = ra_posts(&q.data.lang.code),
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

