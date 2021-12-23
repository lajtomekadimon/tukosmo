use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::languages::LanguagesAResponse;
use crate::handlers::admin::new_language::ra_new_language;
use crate::handlers::admin::edit_language::ra_edit_language_w_id;


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
            routes: &q.routes,
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
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.languages

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }

                a[
                    href = ra_new_language(&q.data.lang.code),
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
                                    href = ra_edit_language_w_id(
                                        &q.data.lang.code,
                                        &lang.id,
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

