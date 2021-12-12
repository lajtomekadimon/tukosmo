use markup;

use crate::i18n::translate_i18n::TranslateI18N;
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
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a PostsAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @if q.filter == "drafts" {
                    @t.draft_posts
                } else if q.filter == "published" {
                    @t.published_posts
                } else if q.filter == "untranslated" {
                    @t.untranslated_posts
                } else if q.filter == "deleted" {
                    @t.deleted_posts
                } else {
                    @t.posts
                }

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        routes: &q.routes,
                        data: &q.data,
                    }
                }

                a[
                    href = "/{lang}/admin/new_post"
                        .replace("{lang}", &q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.new_post
                }
            }

            h2[class = "subtitle"] {
                @t.page_n
                    .replace("{n}", &q.page.to_string())

                " ("
                {(match q.posts.iter().len() {
                    1 => t.one_result_of_m,
                    _ => t.n_results_of_m,
                })
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
                    @t.your_website_posts_were_successfully_updated
                }
            }

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            @t.title
                        }
                        th {
                            @t.status
                        }
                        th {
                            @t.published
                        }
                        th {
                            @t.author
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
                                        @t.deleted_k_post
                                    } else if post.translator == 0 {
                                        @t.untranslated
                                    } else if post.draft {
                                        @t.draft
                                    } else {
                                        @t.published
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
                                        {t.translated_by_user
                                            .replace(
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

            @if &q.total_pages > &1 {
                @AdminPagination {
                    data: &q.data,
                    t: t,
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
}

