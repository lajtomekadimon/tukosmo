use markup;

use crate::config::global::Config;
use crate::handlers::admin::{
    users_get::{
        AgoUsers,
        ra_users_wu_rpp_p,
    },
    scope_users::new_get::ra_users_new,
    scope_users::edit_get::ra_users_edit_w_id,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_pagination::AdminPagination,
};


markup::define! {
    Users<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
        title: &'a str,
        q: &'a AgoUsers,
        t: &'a TranslateI18N,
        success: &'a bool,
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
                },
                current_page: "users",
                codename: codename,
                config: config,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoUsers,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.users

                a[
                    href = ra_users_new(&q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.new_user
                }
            }

            @if &q.total_pages > &1 {
                h2[class = "subtitle"] {
                    {t.page_n
                        .replace("{n}", &q.page.to_string())}

                    " ("
                    {(match q.users.iter().len() {
                        1 => t.one_result_of_m,
                        _ => t.n_results_of_m,
                    })
                        .replace("{n}", &(q.users.iter().len()).to_string())
                        .replace("{m}", &q.total_results.to_string())
                    }
                    ")"
                }
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_users_were_successfully_updated
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            @t.name
                        }
                        th {
                            @t.email
                        }
                        th {
                            @t.account_status
                        }
                        th {
                            @t.date
                        }
                    }
                }
                tbody {
                    @if q.users.len() == 0 {
                        tr {
                            td { "-" }
                            td { "-" }
                            td { "-" }
                            td { "-" }
                        }
                    } else {
                        @for user in q.users.iter() {
                            tr {
                                td {
                                    a[
                                        href = ra_users_edit_w_id(
                                            &q.data.lang.code,
                                            &user.id,
                                        ),
                                    ] {
                                        @user.name
                                    }
                                }
                                td {
                                    @user.email
                                }
                                td {
                                    @if user.suspended {
                                        @t.suspended_k_account
                                    } else {
                                        @if user.admin {
                                            @t.admin
                                        } else {
                                            @t.active_k_account
                                        }
                                    }
                                }
                                td {
                                    {t_date(&user.date, &q.data.lang.code)}
                                }
                            }
                        }
                    }
                }
            }

            @if &q.total_pages > &1 {
                @AdminPagination {
                    data: &q.data,
                    t: t,
                    route: &ra_users_wu_rpp_p(&q.data.lang.code),
                    current_page: &q.page,
                    total_pages: &q.total_pages,
                    results_per_page: &q.results_per_page,
                    buttons: &false,
                }
            }
        }
    }
}

