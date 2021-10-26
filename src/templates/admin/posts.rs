use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::templates::widgets::admin_pagination::AdminPagination;
use crate::handlers::admin::posts::PostsAResponse;


markup::define! {
    Posts<'a>(
        title: &'a str,
        q: &'a PostsAResponse,
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
                current_page: if q.filter == "drafts" {
                    "posts-drafts"
                } else if q.filter == "published" {
                    "posts-published"
                } else if q.filter == "untranslated" {
                    "posts-untranslated"
                } else if q.filter == "deleted" {
                    "posts-deleted"
                } else {
                    "posts"
                },
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a PostsAResponse,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(if q.filter == "drafts" {
                    "Draft posts"
                } else if q.filter == "published" {
                    "Published posts"
                } else if q.filter == "untranslated" {
                    "Untranslated posts"
                } else if q.filter == "deleted" {
                    "Deleted posts"
                } else {
                    "Posts"
                }, &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: if q.filter == "drafts" {
                            "/admin/posts?f=drafts"
                        } else if q.filter == "published" {
                            "/admin/posts?f=published"
                        } else if q.filter == "untranslated" {
                            "/admin/posts?f=untranslated"
                        } else if q.filter == "deleted" {
                            "/admin/posts?f=deleted"
                        } else {
                            "/admin/posts"
                        },
                        data: &q.data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_post"
                        .replace("{lang}", &q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    {&t("New post", &q.data.lang.code)}
                }
            }

            h2[class = "subtitle"] {
                {&t("Page {n}", &q.data.lang.code)
                    .replace("{n}", &q.page.to_string())
                }

                " ("
                {&t(match q.posts.iter().len() {
                    1 => "{n} result of {m}",
                    _ => "{n} results of {m}",
                }, &q.data.lang.code)
                    .replace("{n}", &(q.posts.iter().len()).to_string())
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
                        "Your website posts were successfully updated.",
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
                            {&t("Title", &q.data.lang.code)}
                        }
                        th {
                            {&t("Status", &q.data.lang.code)}
                        }
                        th {
                            {&t("Published", &q.data.lang.code)}
                        }
                        th {
                            {&t("Author", &q.data.lang.code)}
                        }
                    }
                }
                tbody {
                    @if q.posts.len() == 0 {
                        tr {
                            td { "-" }
                            td { "-" }
                            td { "-" }
                            td { "-" }
                        }
                    } else {
                        @for post in q.posts.iter() {
                            tr[
                                class = if post.translator == 0 {
                                    "has-background-danger-light"
                                } else if post.draft {
                                    "has-background-warning-light"
                                } else {
                                    ""
                                },
                            ] {
                                td {
                                    a[
                                        href = "/{lang}/admin/edit_post\
                                                ?id={id}"
                                            .replace(
                                                "{lang}",
                                                &q.data.lang.code,
                                            )
                                            .replace(
                                                "{id}",
                                                &post.id.to_string()
                                            ),
                                        class = if post.translator == 0 {
                                            "has-text-danger"
                                        } else if post.draft {
                                            "has-text-warning-dark"
                                        } else {
                                            ""
                                        },
                                    ] {
                                        @post.title
                                    }
                                }
                                td {
                                    @if post.deleted {
                                        {&t(
                                            "Deleted [post]",
                                            &q.data.lang.code,
                                        )}
                                    } else if post.translator == 0 {
                                        {&t(
                                            "Untranslated",
                                            &q.data.lang.code,
                                        )}
                                    } else if post.draft {
                                        {&t("Draft", &q.data.lang.code)}
                                    } else {
                                        {&t("Published", &q.data.lang.code)}
                                    }
                                }
                                td {
                                    {t_date(&post.date, &q.data.lang.code)}

                                    @if (post.author_name
                                         != post.translator_name)
                                        && (post.translator != 0)
                                    {
                                        " ("
                                        {t_date(
                                            &post.date_trans,
                                            &q.data.lang.code
                                        )}
                                        ")"
                                    }
                                }
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
                                                &post.author.to_string()
                                            ),
                                        class = if post.translator == 0 {
                                            "has-text-danger"
                                        } else if post.draft {
                                            "has-text-warning-dark"
                                        } else {
                                            ""
                                        },
                                    ] {
                                        @post.author_name
                                    }

                                    @if (post.author_name
                                         != post.translator_name)
                                        && (post.translator != 0)
                                    {
                                        " ("
                                        {&t(
                                            "translated by {name}",
                                            &q.data.lang.code
                                        ).replace(
                                            "{name}",
                                            &post.translator_name,
                                        )}
                                        ")"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            @AdminPagination {
                data: &q.data,
                route: if q.filter == "drafts" {
                    "/{lang}/admin/posts?f=drafts&p={page}&rpp={rpp}"
                } else if q.filter == "published" {
                    "/{lang}/admin/posts?f=published&p={page}&rpp={rpp}"
                } else if q.filter == "untranslated" {
                    "/{lang}/admin/posts?f=untranslated&p={page}&rpp={rpp}"
                } else if q.filter == "deleted" {
                    "/{lang}/admin/posts?f=deleted&p={page}&rpp={rpp}"
                } else {
                    "/{lang}/admin/posts?p={page}&rpp={rpp}"
                },
                current_page: &q.page,
                total_pages: &q.total_pages,
                results_per_page: &q.results_per_page,
            }
        }
    }
}

