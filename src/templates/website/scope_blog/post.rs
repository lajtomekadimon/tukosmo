use markup;

use crate::markdown::render_html::render_html;
use crate::handlers::{
    website::scope_blog::post_get::WgoBlogPost,
    admin::scope_posts::edit_get::ra_posts_edit_w_id,
    files_get::r_file,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::{
    website_layout::WebsiteLayout,
    widgets::site::Site,
    widgets::open_graph_meta::ArticleOG,
};


markup::define! {
    Post<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a WgoBlogPost,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            t: t,
            routes: &q.routes,
            og_title: &q.post.title,
            og_description: &q.post.description,
            og_image: &(
                if let Some(fimage) = &q.post.featured_image {
                    "https://{domain}{dir}"
                        .replace("{domain}", domain)
                        .replace(
                            "{dir}",
                            &r_file(&fimage.name),
                        )
                } else { "".to_string() }
            ),
            og_article: &Some(
                ArticleOG {
                    published_time: q.post.date.clone(),
                    modified_time: q.post.date.clone(),
                }
            ),
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
                            src = &r_file(&fimage.name),
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
                            i[
                                class = "eos-icons notranslate",
                                translate = "no",
                            ] {
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
                            i[
                                class = "eos-icons notranslate",
                                translate = "no",
                            ] {
                                "calendar_today"
                            }

                            " "

                            {t_date(&q.post.date, &q.data.lang.code)}
                            /*
                            }
                            */
                        }
                        
                        @if let Some(user) = &q.data.userd {
                            @if user.admin {
                                span[
                                    class = "post-meta-edit",
                                ] {
                                    a[
                                        href = ra_posts_edit_w_id(
                                            &q.data.lang.code,
                                            &q.post.id,
                                        ),
                                    ] {
                                        i[
                                            class = "eos-icons notranslate",
                                            translate = "no",
                                        ] {
                                            "mode_edit"
                                        }

                                        " "

                                        @t.edit
                                    }
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

                @if q.post.tags.iter().len() > 0 {
                    div[
                        class = "post-tags",
                    ] {
                        @t.tags
                        ":"
                        @for tag in q.post.tags.iter() {
                            a[
                                class = "post-tag",
                                href = "",
                            ] {
                                @tag.name
                            }
                        }
                    }
                }
            }
        }
    }
}

