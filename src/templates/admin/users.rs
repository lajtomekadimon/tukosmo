use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::templates::widgets::admin_pagination::AdminPagination;
use crate::handlers::admin::users::UsersAResponse;
use crate::handlers::admin::new_user::ra_new_user;
use crate::handlers::admin::edit_user::ra_edit_user_w_id;
use crate::handlers::admin::users::ra_users_wu_rpp_p;


markup::define! {
    Users<'a>(
        title: &'a str,
        q: &'a UsersAResponse,
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
                current_page: "users",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a UsersAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.users

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }

                a[
                    href = ra_new_user(&q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.new_user
                }
            }

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
                        }
                    } else {
                        @for user in q.users.iter() {
                            tr {
                                td {
                                    a[
                                        href = ra_edit_user_w_id(
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
                }
            }
        }
    }
}

