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
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                },
                current_page: "posts",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a PostsAResponse,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t("Posts", &q.data.lang.code)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: "/admin/posts",
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
                {&t("{n} results of {m}", &q.data.lang.code)
                    .replace("{n}", &(q.posts.iter().len()).to_string())
                    .replace("{m}", &q.total_results.to_string())
                }
                ")"
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
                                    href = "/{lang}/admin/edit_post?id={id}"
                                        .replace("{lang}", &q.data.lang.code)
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
                                @if post.translator == 0 {
                                    {&t("Untranslated", &q.data.lang.code)}
                                } else if post.draft {
                                    {&t("Draft", &q.data.lang.code)}
                                } else {
                                    {&t("Published", &q.data.lang.code)}
                                }
                            }
                            td {
                                {t_date(&post.date, &q.data.lang.code)}

                                @if (post.author_name != post.translator_name)
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
                                    href = "/{lang}/admin/edit_user?id={id}"
                                        .replace("{lang}", &q.data.lang.code)
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

                                @if (post.author_name != post.translator_name)
                                    && (post.translator != 0)
                                {
                                    " ("
                                    {&t(
                                        "translated by {name}",
                                        &q.data.lang.code
                                    ).replace("{name}", &post.translator_name)}
                                    ")"
                                }
                            }
                        }
                    }
                }
            }

            @AdminPagination {
                data: &q.data,
                route: "/{lang}/admin/posts?p={page}&rpp={rpp}",
                current_page: &q.page,
                total_pages: &q.total_pages,
                results_per_page: &q.results_per_page,
            }
        }
    }
}

