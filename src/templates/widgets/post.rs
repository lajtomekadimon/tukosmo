use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::database::s_post_by_lang_permalink::PostDB;
use crate::markdown::render_html::render_html;


markup::define! {
    Post<'a>(
        lang_code: &'a str,
        post: &'a PostDB,
    ) {
        article[
            class = "post",
        ] {
            div[
                class = "post-image",
            ] {
                figure {
                    img[
                        src = "https://www.azamara.com/sites/default/files/heros/reykjavik-iceland-1800x1000.jpg",
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
                            @post.title
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
                                    .replace("{lang}", lang_code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "person"
                                }

                                " "

                                @post.author_name
                            }
                        }
                        
                        span[
                            class = "post-meta-date",
                        ] {
                            a[
                                href = "/{lang}/blog"
                                    .replace("{lang}", lang_code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "calendar_today"
                                }

                                " "

                                {t_date(&post.date, lang_code)}
                            }
                        }
                        
                        span[
                            class = "post-meta-edit",
                        ] {
                            a[
                                href = "/{lang}/blog"
                                    .replace("{lang}", lang_code)
                                ,
                            ] {
                                i[class = "eos-icons"] {
                                    "mode_edit"
                                }

                                " "

                                {&t("Edit", lang_code)}
                            }
                        }
                    }
                }

                div[
                    class = "post-body",
                ] {
                    {markup::raw(&render_html(&post.body))}
                }
            }
        }
    }
}

