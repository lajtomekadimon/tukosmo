use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_post::EditPostAResponse;
use crate::database::types::PostDB;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    EditPost<'a>(
        title: &'a str,
        q: &'a EditPostAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    post: &(q.post).as_ref().unwrap(),
                    error: error,
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
                action = "/{lang}/admin/edit_post"
                    .replace("{lang}", &q.data.lang.code)
                ,
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
                            class = "input",
                            type = "text",
                            name = "title",
                            value = &post.title,
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Permalink", &q.data.lang.code)}
                    }
                    div[class = "control"] {
                        input[
                            class = "input",
                            type = "text",
                            name = "permalink",
                            value = &post.permalink,
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Description", &q.data.lang.code)}
                    }
                    div[class = "control"] {
                        textarea[
                            class = "textarea has-fixed-size",
                            name = "description",
                            rows = "3",
                        ] {
                            @post.description
                        }
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Post's body", &q.data.lang.code)}
                    }
                    div[class = "control"] {
                        textarea[
                            class = "textarea is-family-monospace",
                            name = "body",
                            rows = "12",
                        ] {
                            @post.body
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
                                checked = post.draft,
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
                                checked = post.deleted,
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

