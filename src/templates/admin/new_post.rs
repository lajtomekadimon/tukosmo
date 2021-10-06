use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::new_post::NewPostAResponse;


markup::define! {
    NewPost<'a>(
        title: &'a str,
        q: &'a NewPostAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                },
                current_page: "new_post",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a NewPostAResponse,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("New post", &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/new_post",
                        data: &q.data,
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/new_post"
                    .replace("{lang}", &q.data.lang.code)
                ,
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Title", &q.data.lang.code)}
                    }
                    div[class = "control"] {
                        input[
                            class = "input",
                            type = "text",
                            name = "title",
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
                        ] {}
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
                        ] {}
                    }
                }

                div[class = "field"] {
                    div[class = "control"] {
                        label[class = "checkbox"] {
                            input[
                                type = "checkbox",
                                name = "draft",
                                value = "yes",
                            ];
                            " "
                            {&t("Draft", &q.data.lang.code)}
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

