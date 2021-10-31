use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::delete_post::DeletePostAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    DeletePost<'a>(
        title: &'a str,
        q: &'a DeletePostAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    error: error,
                },
                current_page: "delete_post",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a DeletePostAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.delete_post_w_title
                    .replace("{title}", &q.post.id.to_string())

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/delete_post?id={id}"
                            .replace("{id}", &q.post.id.to_string()),
                        data: &q.data,
                    }
                }
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
                action = "/{lang}/admin/delete_post?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &q.post.id.to_string()),
            ] {
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
                            href = "/{lang}/admin/posts"
                                .replace("{lang}", &q.data.lang.code)
                            ,
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

