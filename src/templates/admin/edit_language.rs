use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::edit_language::EditLanguageAH;


markup::define! {
    EditLanguage<'a>(
        title: &'a str,
        q: &'a EditLanguageAH,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                },
                current_page: "edit_language",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a EditLanguageAH,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit language: {name}",
                    &q.data.lang.code
                ).replace("{name}", &q.lang.name)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/edit_language?id={id}"
                            .replace("{id}", &q.lang.id.to_string()),
                        data: &q.data,
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/edit_language"
                    .replace("{lang}", &q.data.lang.code),
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Code", &q.data.lang.code)}
                    }
                    div[class = "control"] {
                        input[
                            type = "hidden",
                            name = "language_id",
                            value = &q.lang.id.to_string(),
                        ];
                        input[
                            class = "input",
                            type = "text",
                            name = "lang_code",
                            value = &q.lang.code,
                            placeholder = &t("Example: en", &q.data.lang.code),
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Language name", &q.data.lang.code)}
                    }
                    p[class = "control"] {
                        @for name in q.names.iter() {
                            div[class = "field has-addons is-marginless"] {
                                div[class = "control"] {
                                    span[class = "button is-static"] {
                                        @name.lang.name
                                    }
                                }
                                div[class = "control is-expanded"] {
                                    input[
                                        type = "hidden",
                                        name = "lang_id",
                                        value = &name.lang.id.to_string(),
                                    ];
                                    input[
                                        class = "input",
                                        type = "text",
                                        name = "lang_name",
                                        value = &name.name,
                                    ];
                                }
                            }
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
                            href = "/{lang}/admin/languages"
                                .replace("{lang}", &q.data.lang.code),
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

