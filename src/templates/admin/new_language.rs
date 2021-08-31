use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::database::s_languages::s_languages;


markup::define! {
    NewLanguage<'a>(
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
                current_page: "new_language",
                lang_code: lang_code,
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Add language", lang_code)}
            }

            form[
                method = "post",
                action = "/{lang}/admin/new_language".replace("{lang}", lang_code),
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Code", lang_code)}
                    }
                    div[class = "control"] {
                        input[
                            class = "input",
                            type = "text",
                            name = "lang_code",
                            placeholder = &t("Example: en", lang_code),
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Language name", lang_code)}
                    }
                    p[class = "control"] {
                        @for lang in s_languages(lang_code.to_string()) {
                            div[class = "field has-addons is-marginless"] {
                                div[class = "control"] {
                                    span[class = "button is-static"] {
                                        @lang.name
                                    }
                                }
                                div[class = "control is-expanded"] {
                                    input[
                                        type = "hidden",
                                        name = "lang_id",
                                        value = &lang.id.to_string(),
                                    ];
                                    input[
                                        class = "input",
                                        type = "text",
                                        name = "lang_name",
                                    ];
                                }
                            }
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
                            href = "/{lang}/admin/languages".replace("{lang}", lang_code),
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

