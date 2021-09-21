use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::s_languages::s_languages;
use crate::database::types::DataDB;


markup::define! {
    Languages<'a>(
        title: &'a str,
        data: &'a DataDB,
    ) {
        @AdminLayout {
            title: title,
            lang: &data.lang,
            content: AdminPanel {
                content: Content {
                    data: data,
                },
                current_page: "languages",
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a DataDB,
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
                    class = "button is-link is-pulled-right has-text-weight-normal mr-4",
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
                    @for lang in s_languages(data.lang.id) {
                        tr {
                            td {
                                a[
                                    href = "/{lang}/admin/edit_language?id={id}"
                                        .replace("{lang}", &data.lang.code)
                                        .replace(
                                            "{id}",
                                            &lang.id.to_string()
                                        ),
                                ] {
                                    @lang.name
                                }

                                @if !lang.has_all_names {
                                    " (!)"
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

