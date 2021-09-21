use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::types::{AdminDataDB, LanguageDB};


markup::define! {
    NewLanguage<'a>(
        title: &'a str,
        data: &'a AdminDataDB,
        languages: &'a Vec<LanguageDB>,
    ) {
        @AdminLayout {
            title: title,
            data: data,
            content: AdminPanel {
                content: Content {
                    data: data,
                    languages: languages,
                },
                current_page: "new_language",
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a AdminDataDB,
        languages: &'a Vec<LanguageDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Add language", &data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/new_language",
                        data: data,
                    }
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/new_language"
                    .replace("{lang}", &data.lang.code),
            ] {
                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Code", &data.lang.code)}
                    }
                    div[class = "control"] {
                        input[
                            class = "input",
                            type = "text",
                            name = "lang_code",
                            placeholder = &t("Example: en", &data.lang.code),
                        ];
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        {&t("Language name", &data.lang.code)}
                    }
                    p[class = "control"] {
                        @for lang in languages.iter() {
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

