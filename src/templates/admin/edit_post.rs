use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::s_post_by_id_lang::PostDB;
use crate::database::s_user_by_session_lang::UserDB;


markup::define! {
    EditPost<'a>(
        title: &'a str,
        lang_code: &'a str,
        post: &'a PostDB,
        user: &'a UserDB,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {
                    lang_code: lang_code,
                    post: post,
                },
                current_page: "edit_post",
                lang_code: lang_code,
                user: user,
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
        post: &'a PostDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit post: '{title}'",
                    lang_code
                ).replace("{title}", &post.id.to_string())}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        lang_code: lang_code,
                        route: &"/admin/edit_post?id={id}"
                            .replace("{id}", &post.id.to_string()),
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/edit_post"
                    .replace("{lang}", lang_code)
                ,
            ] {
                input[
                    type = "hidden",
                    name = "id",
                    value = &post.id,
                ];

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Title", lang_code)}
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
                        {&t("Permalink", lang_code)}
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
                        {&t("Description", lang_code)}
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
                        {&t("Post's body", lang_code)}
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
                            {&t("Draft", lang_code)}
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
                            {&t("Deleted [post]", lang_code)}
                        }
                    }
                }

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-link"] {
                            {&t("Submit", lang_code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/posts"
                                .replace("{lang}", lang_code)
                            ,
                            class = "button is-link is-light",
                        ] {
                            {&t("Cancel", lang_code)}
                        }
                    }
                }
            }
        }
    }
}

