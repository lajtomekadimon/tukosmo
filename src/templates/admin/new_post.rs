use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::types::AdminDataDB;


markup::define! {
    NewPost<'a>(
        title: &'a str,
        data: &'a AdminDataDB,
    ) {
        @AdminLayout {
            title: title,
            data: data,
            content: AdminPanel {
                content: Content {
                    data: data,
                },
                current_page: "new_post",
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a AdminDataDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("New post", &data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/new_post",
                        data: data,
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/new_post"
                    .replace("{lang}", &data.lang.code)
                ,
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Title", &data.lang.code)}
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
                        {&t("Permalink", &data.lang.code)}
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
                        {&t("Description", &data.lang.code)}
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
                        {&t("Post's body", &data.lang.code)}
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
                            {&t("Draft", &data.lang.code)}
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

