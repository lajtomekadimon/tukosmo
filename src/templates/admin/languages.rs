use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::database::s_languages::s_languages;


markup::define! {
    Languages<'a>(
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
                current_page: "languages",
                lang_code: lang_code,
            },
        }
    }

    Content<'a>(
        lang_code: &'a str,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Languages", lang_code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        lang_code: lang_code,
                        route: "/admin/languages",
                    }
                }

                a[
                    href = "/{lang}/admin/new_language".replace("{lang}", lang_code),
                    class = "button is-link is-pulled-right has-text-weight-normal mr-4",
                ] {
                    {&t("Add language", lang_code)}
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            {&t("Language", lang_code)}
                        }
                        th {
                            {&t("Code", lang_code)}
                        }
                        th {
                            {&t("Last update", lang_code)}
                        }
                    }
                }
                tbody {
                    @for lang in s_languages(lang_code.to_string()) {
                        tr {
                            td {
                                a[
                                    href = "/{lang}/admin/edit_language?id={id}"
                                        .replace("{lang}", lang_code)
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
                                {t_date(&lang.date, lang_code)}
                            }
                        }
                    }
                }
            }
        }
    }
}

