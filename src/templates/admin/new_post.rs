use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;


markup::define! {
    NewPost<'a>(
        title: &'a str,
        lang_code: &'a str,
    ) {
        @AdminLayout {
            title: title,
            lang_code: lang_code,
            content: AdminPanel {
                content: Content {
                    lang_code: lang_code,
                },
                current_page: "new_post",
                lang_code: lang_code,
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("New post", lang_code)}
            }

            form[
                method = "post",
                action = "/{lang}/admin/new_post_post"
                    .replace("{lang}", lang_code)
                ,
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Title", lang_code)}
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
                        {&t("Permalink", lang_code)}
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
                        {&t("Description", lang_code)}
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
                        {&t("Post's body", lang_code)}
                    }
                    div[class = "control"] {
                        textarea[
                            class = "textarea",
                            name = "body",
                            rows = "12",
                        ] {}
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
                            href = "/{lang}/admin/languages"
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

