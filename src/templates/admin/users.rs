use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::templates::widgets::admin_pagination::AdminPagination;
use crate::handlers::admin::users::UsersAResponse;


markup::define! {
    Users<'a>(
        title: &'a str,
        q: &'a UsersAResponse,
        success: &'a bool,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    success: success,
                },
                current_page: "users",
                data: &q.data,
            },
        }
    }


    Content<'a>(
        q: &'a UsersAResponse,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Users", &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/users",
                        data: &q.data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_user"
                        .replace("{lang}", &q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    {&t("New user", &q.data.lang.code)}
                }
            }

            h2[class = "subtitle"] {
                {&t("Page {n}", &q.data.lang.code)
                    .replace("{n}", &q.page.to_string())
                }

                " ("
                {&t(match q.users.iter().len() {
                    1 => "{n} result of {m}",
                    _ => "{n} results of {m}",
                }, &q.data.lang.code)
                    .replace("{n}", &(q.users.iter().len()).to_string())
                    .replace("{m}", &q.total_results.to_string())
                }
                ")"
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    {&t(
                        "Your website users were successfully updated.",
                        &q.data.lang.code
                    )}
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            {&t("Name", &q.data.lang.code)}
                        }
                        th {
                            {&t("Email", &q.data.lang.code)}
                        }
                        th {
                            {&t("Date", &q.data.lang.code)}
                        }
                    }
                }
                tbody {
                    @if q.users.len() == 0 {
                        tr {
                            td { "-" }
                            td { "-" }
                            td { "-" }
                        }
                    } else {
                        @for user in q.users.iter() {
                            tr {
                                td {
                                    a[
                                        href = "/{lang}/admin/edit_user\
                                                ?id={id}"
                                            .replace(
                                                "{lang}",
                                                &q.data.lang.code,
                                            )
                                            .replace(
                                                "{id}",
                                                &user.id.to_string()
                                            ),
                                    ] {
                                        @user.name
                                    }
                                }
                                td {
                                    @user.email
                                }
                                td {
                                    {t_date(&user.date, &q.data.lang.code)}
                                }
                            }
                        }
                    }
                }
            }

            @AdminPagination {
                data: &q.data,
                route: "/{lang}/admin/users?p={page}&rpp={rpp}",
                current_page: &q.page,
                total_pages: &q.total_pages,
                results_per_page: &q.results_per_page,
            }
        }
    }
}

