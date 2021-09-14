use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::awa_edit_language::awa_edit_language;
use crate::database::data::DataDB;


markup::define! {
    EditLanguage<'a>(
        title: &'a str,
        lang_id: &'a i64,
        lang_id_code: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang: &data.lang,
            content: AdminPanel {
                content: Content {
                    lang_id: lang_id,
                    lang_id_code: lang_id_code,
                    data: data,
                },
                current_page: "new_language",
                data: data,
            },
        }
    }

    Content<'a>(
        lang_id: &'a i64,
        lang_id_code: &'a str,
        data: &'a DataDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit language: '{lang}'",
                    &data.lang.code
                ).replace("{lang}", &lang_id_code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/edit_language?id={id}"
                            .replace("{id}", &lang_id.to_string()),
                        data: data,
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/edit_language"
                    .replace("{lang}", &data.lang.code),
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Code", &data.lang.code)}
                    }
                    div[class = "control"] {
                        input[
                            type = "hidden",
                            name = "language_id",
                            value = &lang_id.to_string(),
                        ];
                        input[
                            class = "input",
                            type = "text",
                            name = "lang_code",
                            value = &lang_id_code,
                            placeholder = &t("Example: en", &data.lang.code),
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Language name", &data.lang.code)}
                    }
                    p[class = "control"] {
                        @for name in awa_edit_language(
                            data.lang.id,
                            *lang_id.clone()
                        ) {
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
                                        value = name.name,
                                    ];
                                }
                            }
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
                            href = "/{lang}/admin/languages"
                                .replace("{lang}", &data.lang.code),
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

