use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_post::EditPostAResponse;
use crate::handlers::admin::edit_post_post::FormData;
use crate::database::types::PostDB;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;
use crate::handlers::admin::edit_post::ra_edit_post_w_id;
use crate::handlers::admin::posts::ra_posts;
use crate::handlers::admin::delete_post::ra_delete_post_w_id;


markup::define! {
    EditPost<'a>(
        title: &'a str,
        q: &'a EditPostAResponse,
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
                    post: &(q.post).as_ref().unwrap(),
                    error: error,
                    form: form,
                },
                current_page: "edit_post",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a EditPostAResponse,
        t: &'a TranslateI18N,
        post: &'a PostDB,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.edit_post_w_title
                    .replace("{title}", &post.id.to_string())

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
                action = ra_edit_post_w_id(
                    &q.data.lang.code,
                    &post.id,
                ),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                input[
                    type = "hidden",
                    name = "id",
                    value = &post.id,
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
                            } else { &post.title },
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.featured_image
                    }
                    div[class = "control"] {
                        @if let Some(fimage) = &q.featured_image {
                            input[
                                class = if let Some(e) = error {
                                    if e.code == ec::WRONG_FILE_ID {
                                        "input is-danger"
                                    } else {
                                        "input"
                                    }
                                } else {
                                    "input"
                                },
                                type = "text",
                                name = "featured_image",
                                value = if let Some(f) = form {
                                    &f.featured_image
                                } else { &fimage.id },
                            ];
                        } else {
                            input[
                                class = if let Some(e) = error {
                                    if e.code == ec::WRONG_FILE_ID {
                                        "input is-danger"
                                    } else {
                                        "input"
                                    }
                                } else {
                                    "input"
                                },
                                type = "text",
                                name = "featured_image",
                                value = if let Some(f) = form {
                                    &f.featured_image
                                } else { &0 },
                            ];
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
                            } else { &post.permalink },
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
                            } else { @post.description }
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.post_s_body
                    }
                    div[class = "control"] {
                        textarea[
                            class = if let Some(e) = error {
                                if e.code == ec::WRONG_BODY_TEXT {
                                    "textarea is-family-monospace is-danger"
                                } else {
                                    "textarea is-family-monospace"
                                }
                            } else {
                                "textarea is-family-monospace"
                            },
                            name = "body",
                            rows = "12",
                        ] {
                            @if let Some(f) = form {
                                {&f.body}
                            } else { @post.body }
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
                                } else { post.draft },
                            ];
                            " "
                            @t.draft
                        }
                    }
                }

                // TODO: Hide this field when creating a translation
                div[class = "field"] {
                    div[class = "control"] {
                        label[class = "checkbox"] {
                            input[
                                type = "checkbox",
                                name = "deleted",
                                value = "yes",
                                checked = if let Some(f) = form {
                                    match &f.deleted {
                                        Some(_) => true,
                                        None => false,
                                    }
                                } else { post.deleted },
                            ];
                            " "
                            @t.deleted_k_post
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

                    div[class = "control"] {
                        a[
                            href = ra_delete_post_w_id(
                                &q.data.lang.code,
                                &post.id,
                            ),
                            class = "button is-danger \
                                     has-text-weight-normal mr-4",
                        ] {
                            @if post.deleted {
                                @t.delete_permanent
                            } else {
                                @t.delete
                            }
                        }
                    }
                }
            }
        }
    }
}

