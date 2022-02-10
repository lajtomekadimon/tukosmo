use markup;

use crate::handlers::admin::sessions_get::{
    AgoSessions,
    ra_sessions,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
    t_error::ErrorDB,
    get_browser_from_user_agent::get_browser_from_user_agent,
    get_os_from_user_agent::get_os_from_user_agent,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
};


markup::define! {
    Sessions<'a>(
        domain: &'a str,
        title: &'a str,
        q: &'a AgoSessions,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            domain: domain,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    success: success,
                    error: error,
                },
                current_page: "sessions",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoSessions,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.sessions

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_sessions_have_been_successfully_updated
                }
            }

            @if let Some(e) = error {
                div[
                    class = "notification is-danger",
                ] {
                    button[class = "delete"] {}
                    @e.message
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            @t.browser
                        }
                        th {
                            @t.system_k_os
                        }
                        th {
                            @t.since
                        }
                        th {
                            @t.close_k_session
                        }
                    }
                }
                tbody {
                    @for sessiond in q.sessions.iter() {
                        tr {
                            td {
                                {&get_browser_from_user_agent(
                                    &sessiond.user_agent
                                )}
                            }
                            td {
                                {&get_os_from_user_agent(
                                    &sessiond.user_agent
                                )}
                            }
                            td {
                                {t_date(&sessiond.date, &q.data.lang.code)}
                            }
                            td {
                                form[
                                    action = ra_sessions(&q.data.lang.code),
                                    method = "post",
                                ] {
                                    input[
                                        type = "hidden",
                                        name = "csrf_token",
                                        value = &q.csrf_token,
                                    ];

                                    input[
                                        type = "hidden",
                                        name = "user_agent",
                                        value = &sessiond.user_agent,
                                    ];

                                    input[
                                        type = "hidden",
                                        name = "date",
                                        value = &sessiond.date,
                                    ];

                                    button[
                                        class = "button is-danger is-small",
                                        type = "submit",
                                        title = t.logout_k_verb,
                                    ] {
                                        i[
                                            class = "eos-icons notranslate \
                                                     is-size-5",
                                            translate = "no",
                                        ] {
                                            "logout"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

