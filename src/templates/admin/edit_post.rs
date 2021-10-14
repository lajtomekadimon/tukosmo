use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_post::EditPostAResponse;
use crate::handlers::admin::edit_post_post::FormData;
use crate::database::types::PostDB;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    EditPost<'a>(
        title: &'a str,
        q: &'a EditPostAResponse,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    post: &(q.post).as_ref().unwrap(),
                    error: error,
                    form: form,
                },
                current_page: "edit_post",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a EditPostAResponse,
        post: &'a PostDB,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit post: '{title}'",
                    &q.data.lang.code
                ).replace("{title}", &post.id.to_string())}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/edit_post?id={id}"
                            .replace("{id}", &post.id.to_string()),
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
                action = "/{lang}/admin/edit_post?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &post.id.to_string()),
            ] {
                input[
                    type = "hidden",
                    name = "id",
                    value = &post.id,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Title", &q.data.lang.code)}
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
                        {&t("Permalink", &q.data.lang.code)}
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
                        {&t("Description", &q.data.lang.code)}
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
                        {&t("Post's body", &q.data.lang.code)}
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
                            {&t("Draft", &q.data.lang.code)}
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
                            {&t("Deleted [post]", &q.data.lang.code)}
                        }
                    }
                }

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            {&t("Submit", &q.data.lang.code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/posts"
                                .replace("{lang}", &q.data.lang.code)
                            ,
                            class = "button is-link is-light",
                        ] {
                            {&t("Cancel", &q.data.lang.code)}
                        }
                    }
                }
            }
        }
    }
}

