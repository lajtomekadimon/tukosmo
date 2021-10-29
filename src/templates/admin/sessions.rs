use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::sessions::SessionsAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    Sessions<'a>(
        title: &'a str,
        q: &'a SessionsAResponse,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    success: success,
                    error: error,
                },
                current_page: "sessions",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a SessionsAResponse,
        success: &'a bool,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Sessions", &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/sessions",
                        data: &q.data,
                    }
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    {&t(
                        "Your sessions have been successfully updated.",
                        &q.data.lang.code
                    )}
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
                            {&t("Device", &q.data.lang.code)}
                        }
                        th {
                            {&t("Since", &q.data.lang.code)}
                        }
                        th {
                            {&t("Close [session]", &q.data.lang.code)}
                        }
                    }
                }
                tbody {
                    @for sessiond in q.sessions.iter() {
                        tr {
                            td {
                                @sessiond.user_agent
                            }
                            td {
                                {t_date(&sessiond.date, &q.data.lang.code)}
                            }
                            td {
                                form[
                                    action = "/{lang}/admin/sessions"
                                        .replace("{lang}", &q.data.lang.code),
                                    method = "post",
                                ] {
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
                                        title = &t(
                                            "Logout [verb]",
                                            &q.data.lang.code,
                                        ),
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

