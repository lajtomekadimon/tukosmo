use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::types::{AdminDataDB, LanguageDB};


markup::define! {
    Languages<'a>(
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
                current_page: "languages",
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
                {&t("Languages", &data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/languages",
                        data: data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_language"
                        .replace("{lang}", &data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    {&t("Add language", &data.lang.code)}
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            {&t("Language", &data.lang.code)}
                        }
                        th {
                            {&t("Code", &data.lang.code)}
                        }
                        th {
                            {&t("Last update", &data.lang.code)}
                        }
                    }
                }
                tbody {
                    @for lang in languages.iter() {
                        tr[
                            class = if !lang.has_all_names {
                                "has-background-danger-light"
                            } else {
                                ""
                            },
                        ] {
                            td {
                                a[
                                    href = "/{lang}/admin/edit_language\
                                            ?id={id}"
                                        .replace("{lang}", &data.lang.code)
                                        .replace(
                                            "{id}",
                                            &lang.id.to_string()
                                        ),
                                    class = if !lang.has_all_names {
                                        "has-text-danger"
                                    } else {
                                        ""
                                    },
                                ] {
                                    @lang.name

                                    @if lang.id != data.lang.id {
                                        " ("
                                        @lang.original_name
                                        ")"
                                    }
                                }
                            }
                            td {
                                @lang.code
                            }
                            td {
                                {t_date(&lang.date, &data.lang.code)}
                            }
                        }
                    }
                }
            }
        }
    }
}

