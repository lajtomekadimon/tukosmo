use actix_web::web::Form as ActixForm;
use markup;

use crate::handlers::admin::{
    scope_posts::edit_get::{
        AgoPostsEdit,
        ra_posts_edit_w_id,
    },
    scope_posts::edit_post::FormData,
    scope_posts::delete_get::ra_posts_delete_w_id,
    posts_get::ra_posts,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::database::{
    types::PostDB,
    error_codes as ec,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_post_editor::AdminPostEditor,
    widgets::admin_file_selector::AdminFileSelector,
};


markup::define! {
    Edit<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoPostsEdit,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
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
                    post: &(q.post).as_ref().unwrap(),
                    error: error,
                    form: form,
                },
                current_page: "edit_post",
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoPostsEdit,
        t: &'a TranslateI18N,
        post: &'a PostDB,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.edit_post_w_title
                    .replace("{title}", &post.id.to_string())
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
                action = ra_posts_edit_w_id(
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
                        @AdminFileSelector {
                            t: t,
                            name: "featured_image",
                            current_file: &q.featured_image,
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
                        @AdminPostEditor {
                            name: "body",
                            init_value: if let Some(f) = form {
                                &f.body
                            } else {
                                &post.body
                            },
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.tags
                    }
                    div[class = "control"] {
                        div[class = "field has-addons"] {
                            div[class = "control is-expanded"] {
                                div[class = "select is-fullwidth"] {
                                    select[
                                        id = "ttags-list",
                                    ] {
                                        option[
                                            selected = true,
                                            disabled = true,
                                            hidden = true,
                                        ] {
                                            @t.select_a_tag
                                        }
                                        @for tag in q.tags.iter() {
                                            option[
                                                value = &tag.id,
                                            ] {
                                                @tag.name
                                            }
                                        }
                                    }
                                }
                            }
                            div[class = "control"] {
                                button[
                                    id = "ttag-add-button",
                                    class = "button is-link",
                                    type = "button",
                                ] {
                                    @t.add_k_verb
                                }
                            }
                        }

                        div[
                            id = "ttags-added",
                            class = "field is-grouped is-grouped-multiline",
                        ] {
                            @if let Some(f) = form {
                                @for tag in q.tags.iter() {
                                    @if f.tags.contains(&tag.id) {
                                        input[
                                            id = "tag-input-{id}"
                                                .replace(
                                                    "{id}",
                                                    &tag.id.to_string(),
                                                ),
                                            type = "hidden",
                                            name = "tag",
                                            value = &tag.id,
                                        ];

                                        div[
                                            id = "tag-label-{id}"
                                                .replace(
                                                    "{id}",
                                                    &tag.id.to_string(),
                                                ),
                                            class = "control",
                                        ] {
                                            div[class = "tags has-addons"] {
                                                a[
                                                    class = "tag is-link",
                                                ] {
                                                    @tag.name
                                                }
                                                a[
                                                    class = "tag is-delete \
                                                             ttag-remove",
                                                    "data-id" = &tag.id,
                                                ] {}
                                            }
                                        }
                                    }
                                }
                            } else {
                                @for tag in q.tags_of_post.iter() {
                                    input[
                                        id = "tag-input-{id}"
                                            .replace(
                                                "{id}",
                                                &tag.id.to_string(),
                                            ),
                                        type = "hidden",
                                        name = "tag",
                                        value = &tag.id,
                                    ];

                                    div[
                                        id = "tag-label-{id}"
                                            .replace(
                                                "{id}",
                                                &tag.id.to_string(),
                                            ),
                                        class = "control",
                                    ] {
                                        div[class = "tags has-addons"] {
                                            a[
                                                class = "tag is-link",
                                            ] {
                                                @tag.name
                                            }
                                            a[
                                                class = "tag is-delete \
                                                         ttag-remove",
                                                "data-id" = &tag.id,
                                            ] {}
                                        }
                                    }
                                }
                            }
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
                            href = ra_posts_delete_w_id(
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

