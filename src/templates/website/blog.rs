use markup;

use crate::handlers::{
    website::{
        blog_get::{
            WgoBlog,
            rw_blog_wu_rpp_p,
        },
        scope_blog::post_get::rw_blog_post,
    },
    files_get::r_file,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::{
    website_layout::WebsiteLayout,
    widgets::{
        site::Site,
        pagination::Pagination,
        icons,
    },
};


markup::define! {
    Blog<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a WgoBlog,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            t: t,
            routes: &q.routes,
            og_title: &t.blog,
            og_description: "",
            og_image: "",
            og_article: &None,
            content: Site {
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
        q: &'a WgoBlog,
        t: &'a TranslateI18N,
    ) {
        div[
            class = "blog",
        ] {
            @for post in q.posts.iter() {
                section[
                    class = "blog-post"
                ] {
                    @if let Some(fimage) = &post.featured_image {
                        div[class = "blog-post-image"] {
                            a[
                                href = rw_blog_post(
                                    &post.lang.code,
                                    &post.permalink,
                                ),
                            ] {
                                figure[
                                    style =
                                        "background-image: url({url});"
                                            .replace(
                                                "{url}",
                                                &r_file(
                                                    &fimage.name,
                                                ),
                                            ),
                                ] {}
                            }
                        }
                    }

                    div[
                        class = if let Some(_fimage) = &post.featured_image {
                            "blog-post-data"
                        } else {
                            "blog-post-data blog-post-data-noimg"
                        }
                    ] {
                        div[
                            class = "blog-post-data-meta",
                        ] {
                            div[
                                class = "blog-post-data-meta-date",
                            ] {
                                /*
                                a[
                                    href = rw_blog_post(
                                        &post.lang.code,
                                        &post.permalink,
                                    ),
                                ] {
                                */
                                @icons::CalendarToday {}

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
                                /*
                                }
                                */
                            }
                        }

                        h2 {
                            a[
                                href = rw_blog_post(
                                    &post.lang.code,
                                    &post.permalink,
                                ),
                            ] {
                                @post.title
                            }
                        }

                        p {
                            @post.description
                        }

                        div[
                            class = "blog-post-data-more",
                        ] {
                            a[
                                href = rw_blog_post(
                                    &post.lang.code,
                                    &post.permalink,
                                ),
                            ] {
                                @t.read_more
                            }
                        }
                    }
                }
            }
        }

        @if &q.total_pages > &1 {
            @Pagination {
                data: &q.data,
                t: t,
                route: &rw_blog_wu_rpp_p(&q.data.lang.code),
                current_page: &q.page,
                total_pages: &q.total_pages,
                results_per_page: &q.results_per_page,
            }
        }
    }
}

