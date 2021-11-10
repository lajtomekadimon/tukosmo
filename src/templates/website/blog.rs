use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::templates::widgets::blog_pagination::BlogPagination;
use crate::handlers::website::blog::BlogWResponse;


markup::define! {
    Blog<'a>(
        title: &'a str,
        q: &'a BlogWResponse,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {
                    q: q,
                    t: t,
                },
                routes: &q.routes,
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a BlogWResponse,
        t: &'a TranslateI18N,
    ) {
        div[
            class = "post-list",
        ] {
            @for post in q.posts.iter() {
                section[
                    class = "post-wrapper"
                ] {
                    div[
                        class = "post-wrapper-image"
                    ] {
                        a[
                            href = "/{lang}/blog/{permalink}"
                                .replace("{lang}", &post.lang.code)
                                .replace(
                                    "{permalink}",
                                    &post.permalink.to_string()
                                ),
                        ] {
                            figure[
                                style = "background-image: url(https://www.\
                                         azamara.com/sites/default/files/\
                                         heros/reykjavik-iceland-\
                                         1800x1000.jpg);",
                            ] {}
                        }
                    }

                    div[
                        class = "post-wrapper-data",
                    ] {
                        div[
                            class = "post-wrapper-data-meta",
                        ] {
                            div[
                                class = "post-wrapper-data-meta-date",
                            ] {
                                a[
                                    href = "/{lang}/blog/{permalink}"
                                        .replace("{lang}", &post.lang.code)
                                        .replace(
                                            "{permalink}",
                                            &post.permalink.to_string()
                                        ),
                                ] {
                                    i[class = "eos-icons"] {
                                        "calendar_today"
                                    }

                                    " "

                                    time[
                                        datetime = "2021-08-11T20:37:29+00:00",
                                    ] {
                                        {t_date(&post.date, &q.data.lang.code)}
                                    }

                                    @if post.lang.id != q.data.lang.id {
                                        " "

                                        b {
                                            "("
                                            @t.untranslated_k_lower
                                            ")"
                                        }
                                    }
                                }
                            }
                        }

                        h2 {
                            a[
                                href = "/{lang}/blog/{permalink}"
                                    .replace("{lang}", &post.lang.code)
                                    .replace(
                                        "{permalink}",
                                        &post.permalink.to_string()
                                    ),
                            ] {
                                @post.title
                            }
                        }

                        p {
                            @post.description
                        }

                        div[
                            class = "post-wrapper-data-more",
                        ] {
                            a[
                                href = "/{lang}/blog/{permalink}"
                                    .replace("{lang}", &post.lang.code)
                                    .replace(
                                        "{permalink}",
                                        &post.permalink.to_string()
                                    ),
                            ] {
                                @t.read_more
                            }
                        }
                    }
                }
            }
        }

        @BlogPagination {
            data: &q.data,
            t: t,
            route: "/{lang}/blog?p={page}&rpp={rpp}",
            current_page: &q.page,
            total_pages: &q.total_pages,
            results_per_page: &q.results_per_page,
        }
    }
}

