use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::types::{AdminDataDB, PostDB};


markup::define! {
    EditPost<'a>(
        title: &'a str,
        post: &'a PostDB,
        data: &'a AdminDataDB,
    ) {
        @AdminLayout {
            title: title,
            data: data,
            content: AdminPanel {
                content: Content {
                    post: post,
                    data: data,
                },
                current_page: "edit_post",
                data: data,
            },
        }
    }

    Content<'a>(
        post: &'a PostDB,
        data: &'a AdminDataDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit post: '{title}'",
                    &data.lang.code
                ).replace("{title}", &post.id.to_string())}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/edit_post?id={id}"
                            .replace("{id}", &post.id.to_string()),
                        data: data,
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/edit_post"
                    .replace("{lang}", &data.lang.code)
                ,
            ] {
                input[
                    type = "hidden",
                    name = "id",
                    value = &post.id,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Title", &data.lang.code)}
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
                        {&t("Permalink", &data.lang.code)}
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
                        {&t("Description", &data.lang.code)}
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
                        {&t("Post's body", &data.lang.code)}
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
                            {&t("Draft", &data.lang.code)}
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
                            {&t("Deleted [post]", &data.lang.code)}
                        }
                    }
                }

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            {&t("Submit", &data.lang.code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/posts"
                                .replace("{lang}", &data.lang.code)
                            ,
                            class = "button is-link is-light",
                        ] {
                            {&t("Cancel", &data.lang.code)}
                        }
                    }
                }
            }
        }
    }
}

