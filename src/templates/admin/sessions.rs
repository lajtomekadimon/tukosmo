use markup;

use crate::handlers::admin::sessions_get::{
    AgoSessions,
    ra_sessions,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Sessions<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoSessions,
        t: &'a TranslateI18N,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
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
                codename: codename,
                data: &q.data,
                t: t,
                routes: &q.routes,
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
                            "IP"
                        }
                        th {
                            @t.browser
                        }
                        th {
                            @t.platform_k_os
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
                                @sessiond.ip
                            }
                            td {
                                @sessiond.browser
                            }
                            td {
                                @sessiond.platform
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
                                        name = "ip",
                                        value = &sessiond.ip,
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

