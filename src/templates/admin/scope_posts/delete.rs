use markup;

use crate::config::global::Config;
use crate::handlers::admin::{
    scope_posts::delete_get::{
        AgoPostsDelete,
        ra_posts_delete_w_id,
    },
    posts_get::ra_posts,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_error::ErrorDB,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Delete<'a>(
        domain: &'a str,
        codename: &'a str,
        config: &'a Config,
        title: &'a str,
        q: &'a AgoPostsDelete,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    error: error,
                },
                current_page: "delete_post",
                codename: codename,
                config: config,
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoPostsDelete,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.delete_post_w_title
                    .replace("{title}", &q.post.id.to_string())
            }

            @if let Some(e) = error {
                div[
                    class = "notification is-danger",
                ] {
                    button[class = "delete"] {}
                    @e.message
                }
            }

            div[class = "content"] {
                p {
                    @t.are_you_sure_that_you_want_to_delete_this_post
                }
                p {
                    b[
                        class = if q.post.deleted {
                            "has-text-danger"
                        } else {
                            ""
                        },
                    ] {
                        @if q.post.deleted {
                            @t.the_post_will_be_permanently_deleted
                        } else {
                            @t.the_post_will_be_sent_to_trash
                        }
                    }
                }
            }

            form[
                method = "post",
                action = ra_posts_delete_w_id(
                    &q.data.lang.code,
                    &q.post.id,
                ),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                input[
                    type = "hidden",
                    name = "id",
                    value = &q.post.id,
                ];

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-danger"] {
                            @t.delete
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = ra_posts(&q.data.lang.code),
                            class = "button is-link is-light",
                        ] {
                            @t.cancel
                        }
                    }
                }
            }
        }
    }
}

