use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::types::{AdminDataDB, LanguageWithNamesDB};


markup::define! {
    EditLanguage<'a>(
        title: &'a str,
        data: &'a AdminDataDB,
        lang_wnames: &'a LanguageWithNamesDB,
    ) {
        @AdminLayout {
            title: title,
            data: data,
            content: AdminPanel {
                content: Content {
                    data: data,
                    lang_wnames: lang_wnames,
                },
                current_page: "edit_language",
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a AdminDataDB,
        lang_wnames: &'a LanguageWithNamesDB,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Edit language: {name}",
                    &data.lang.code
                ).replace("{name}", &lang_wnames.name)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/edit_language?id={id}"
                            .replace("{id}", &lang_wnames.id.to_string()),
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
                            value = &lang_wnames.id.to_string(),
                        ];
                        input[
                            class = "input",
                            type = "text",
                            name = "lang_code",
                            value = &lang_wnames.code,
                            placeholder = &t("Example: en", &data.lang.code),
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Language name", &data.lang.code)}
                    }
                    p[class = "control"] {
                        @for name in lang_wnames.names.iter() {
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

