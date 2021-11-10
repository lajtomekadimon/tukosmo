use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::languages::LanguagesAResponse;


markup::define! {
    Languages<'a>(
        title: &'a str,
        q: &'a LanguagesAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    success: success,
                },
                current_page: "languages",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a LanguagesAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.languages

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        routes: &q.routes,
                        data: &q.data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_language"
                        .replace("{lang}", &q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.add_language
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_languages_were_successfully_updated
                }
            }

            @if q.some_lang_without_names {
                div[
                    class = "notification is-danger",
                ] {
                    button[class = "delete"] {}
                    @t.there_are_languages_without_names
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            @t.language
                        }
                        th {
                            @t.code
                        }
                        th {
                            @t.last_update
                        }
                    }
                }
                tbody {
                    @for lang in q.data.languages.iter() {
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

