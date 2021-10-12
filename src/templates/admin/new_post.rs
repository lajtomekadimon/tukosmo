use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::new_post::NewPostAResponse;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    NewPost<'a>(
        title: &'a str,
        q: &'a NewPostAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    error: error,
                },
                current_page: "new_post",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a NewPostAResponse,
        error: &'a Option<ErrorDB>,
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
                        ] {}
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

