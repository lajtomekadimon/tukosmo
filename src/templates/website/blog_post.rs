use markup;

use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::blog_post::BlogPostWResponse;
use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::markdown::render_html::render_html;


markup::define! {
    BlogPost<'a>(
        title: &'a str,
        q: &'a BlogPostWResponse,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {
                    q: q,
                },
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a BlogPostWResponse,
    ) {
        article[
            class = "post",
        ] {
            div[
                class = "post-image",
            ] {
                figure {
                    img[
                        src = "https://www.azamara.com/sites/default/\
                               files/heros/reykjavik-iceland-1800x1000.jpg",
                        alt = "reykjavik",
                    ];
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

                                    {&t("Edit", &q.data.lang.code)}
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

