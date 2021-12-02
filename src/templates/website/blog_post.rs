use markup;

use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::blog_post::BlogPostWResponse;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::markdown::render_html::render_html;


markup::define! {
    BlogPost<'a>(
        title: &'a str,
        q: &'a BlogPostWResponse,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
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
        q: &'a BlogPostWResponse,
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
                            src = "/files/{filename}".replace(
                                "{filename}",
                                &fimage.name,
                            ),
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
                            a[
                                href = "/{lang}/blog"
                                    .replace("{lang}", &q.data.lang.code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "person"
                                }

                                " "

                                @q.post.author_name
                            }
                        }
                        
                        span[
                            class = "post-meta-date",
                        ] {
                            a[
                                href = "/{lang}/blog"
                                    .replace("{lang}", &q.data.lang.code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "calendar_today"
                                }

                                " "

                                {t_date(&q.post.date, &q.data.lang.code)}
                            }
                        }
                        
                        @if let Some(_user) = &q.data.userd {
                            span[
                                class = "post-meta-edit",
                            ] {
                                a[
                                    href = "/{lang}/admin/edit_post?id={id}"
                                        .replace("{lang}", &q.data.lang.code)
                                        .replace(
                                            "{id}",
                                            &q.post.id.to_string()
                                        )
                                    ,
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

