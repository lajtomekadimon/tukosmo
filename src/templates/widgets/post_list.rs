use markup;

use crate::i18n::t::t;
use crate::i18n::t_date::t_date;
use crate::database::types::{WebsiteDataDB, PostDB};

/* TODO:
 * - Show something when no articles
 * - Pagination
 */


markup::define! {
    PostList<'a>(
        data: &'a WebsiteDataDB,
        posts: &'a Vec<PostDB>,
    ) {
        div[
            class = "post-list",
        ] {
            @for post in posts.iter() {
                section[
                    class = "post-wrapper"
                ] {
                    div[
                        class = "post-wrapper-image"
                    ] {
                        a[
                            href = "/{lang}/blog/{permalink}"
                                .replace("{lang}", &data.lang.code)
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
                                        .replace("{lang}", &data.lang.code)
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
                                        {t_date(&post.date, &data.lang.code)}
                                    }
                                }
                            }
                        }

                        h2 {
                            a[
                                href = "/{lang}/blog/{permalink}"
                                    .replace("{lang}", &data.lang.code)
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
                                    .replace("{lang}", &data.lang.code)
                                    .replace(
                                        "{permalink}",
                                        &post.permalink.to_string()
                                    ),
                            ] {
                                {&t("Read more", &data.lang.code)}
                            }
                        }
                    }
                }
            }
        }
    }
}

