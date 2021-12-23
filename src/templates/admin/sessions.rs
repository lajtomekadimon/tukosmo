use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::sessions::SessionsAResponse;
use crate::i18n::t_error::ErrorDB;
use crate::i18n::get_browser_from_user_agent::get_browser_from_user_agent;
use crate::i18n::get_os_from_user_agent::get_os_from_user_agent;
use crate::handlers::admin::sessions::ra_sessions;


markup::define! {
    Sessions<'a>(
        title: &'a str,
        q: &'a SessionsAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
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
                    error: error,
                },
                current_page: "sessions",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a SessionsAResponse,
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
                                        i[class = "eos-icons is-size-5"] {
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

