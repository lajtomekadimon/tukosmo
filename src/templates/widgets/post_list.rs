use markup;

use crate::i18n::t::t;
use crate::database::s_posts_by_lang::s_posts_by_lang;

/* TODO:
 * - Show something when no articles
 * - Pagination
 */


markup::define! {
    PostList<'a>(
        lang_code: &'a str,
    ) {
        div[
            class = "post-list",
        ] {
            @for post in s_posts_by_lang(lang_code.to_string()) {
                section[
                    class = "post-wrapper"
                ] {
                    div[
                        class = "post-wrapper-image"
                    ] {
                        a[
                            href = "/{lang}/blog/{permalink}"
                                .replace("{lang}", lang_code)
                                .replace("{permalink}", &post.permalink.to_string()),
                        ] {
                            figure[
                                style = "background-image: url(https://www.azamara.com/sites/default/files/heros/reykjavik-iceland-1800x1000.jpg);",
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
                                        .replace("{lang}", lang_code)
                                        .replace("{permalink}", &post.permalink.to_string()),
                                ] {
                                    time[
                                        datetime = "2021-08-11T20:37:29+00:00",
                                    ] {
                                        // TODO: "August 1, 2021"
                                        @post.date
                                    }
                                }
                            }
                        }

                        h2 {
                            a[
                                href = "/{lang}/blog/{permalink}"
                                    .replace("{lang}", lang_code)
                                    .replace("{permalink}", &post.permalink.to_string()),
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
                                    .replace("{lang}", lang_code)
                                    .replace("{permalink}", &post.permalink.to_string()),
                            ] {
                                {&t("Read more", lang_code)}
                            }
                        }
                    }
                }
            }
        }
    }
}

