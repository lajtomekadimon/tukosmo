use markup;

use crate::markdown::render_html::render_html;
use crate::files::file_route::file_route;
use crate::handlers::{
    website::scope_blog::post_get::WgoBlogPost,
    admin::scope_posts::edit_get::ra_posts_edit_w_id,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::{
    website_layout::WebsiteLayout,
    widgets::website::Website,
    widgets::open_graph_meta::ArticleOG,
};


markup::define! {
    Post<'a>(
        title: &'a str,
        q: &'a WgoBlogPost,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            og_title: &q.post.title,
            og_description: &q.post.description,
            og_image: &(
                if let Some(fimage) = &q.post.featured_image {
                    "https://tukosmo.org{dir}"  // TODO: domain
                        .replace(
                            "{dir}",
                            &file_route(&fimage.name)
                        )

                } else { "".to_string() }
            ),
            og_article: &Some(
                ArticleOG {
                    published_time: q.post.date.clone(),
                    modified_time: q.post.date.clone(),
                }
            ),
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
        q: &'a WgoBlogPost,
        t: &'a TranslateI18N,
    ) {
        article[
            class = "post",
        ] {
            @if let Some(fimage) = &q.post.featured_image {
                div[
                    class = "post-image",
                ] {
                    figure {
                        img[
                            src = &file_route(&fimage.name),
                            alt = &fimage.name,
                        ];
                    }
                }
            }

            div[
                class = "post-content",
            ] {
                div[
                    class = "post-header",
                ] {
                    div[
                        class = "post-title",
                    ] {
                        h1[
                            class = "post-title",
                        ] {
                            @q.post.title
                        }
                    }

                    div[
                        class = "post-meta",
                    ] {
                        span[
                            class = "post-meta-author",
                        ] {
                            /*
                            a[
                                href = rw_blog(&q.data.lang.code),
                            ] {
                            */
                            i[class = "eos-icons"] {
                                "person"
                            }

                            " "

                            @q.post.author_name
                            /*
                            }
                            */
                        }
                        
                        span[
                            class = "post-meta-date",
                        ] {
                            /*
                            a[
                                href = rw_blog(&q.data.lang.code),
                            ] {
                            */
                            i[class = "eos-icons"] {
                                "calendar_today"
                            }

                            " "

                            {t_date(&q.post.date, &q.data.lang.code)}
                            /*
                            }
                            */
                        }
                        
                        @if let Some(_user) = &q.data.userd {
                            span[
                                class = "post-meta-edit",
                            ] {
                                a[
                                    href = ra_posts_edit_w_id(
                                        &q.data.lang.code,
                                        &q.post.id,
                                    ),
                                ] {
                                    i[class = "eos-icons"] {
                                        "mode_edit"
                                    }

                                    " "

                                    @t.edit
                                }
                            }
                        }
                    }
                }

                div[
                    class = "post-body",
                ] {
                    {markup::raw(&render_html(&q.post.body))}
                }
            }
        }
    }
}
