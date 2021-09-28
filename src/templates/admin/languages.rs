use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::languages::LanguagesAResponse;


markup::define! {
    Languages<'a>(
        title: &'a str,
        q: &'a LanguagesAResponse,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                },
                current_page: "languages",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a LanguagesAResponse,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Languages", &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/languages",
                        data: &q.data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_language"
                        .replace("{lang}", &q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    {&t("Add language", &q.data.lang.code)}
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            {&t("Language", &q.data.lang.code)}
                        }
                        th {
                            {&t("Code", &q.data.lang.code)}
                        }
                        th {
                            {&t("Last update", &q.data.lang.code)}
                        }
                    }
                }
                tbody {
                    @for lang in q.languages.iter() {
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
                                        .replace("{lang}", &q.data.lang.code)
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

                                    @if lang.id != q.data.lang.id {
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
                                {t_date(&lang.date, &q.data.lang.code)}
                            }
                        }
                    }
                }
            }
        }
    }
}

