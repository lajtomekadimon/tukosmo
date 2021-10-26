use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::delete_post::DeletePostAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    DeletePost<'a>(
        title: &'a str,
        q: &'a DeletePostAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            content: AdminPanel {
                content: Content {
                    q: q,
                    error: error,
                },
                current_page: "edit_post",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a DeletePostAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Delete post: '{title}'",
                    &q.data.lang.code
                ).replace("{title}", &q.post.id.to_string())}

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
                    {&t(
                        "Are you sure that you want to delete this post?",
                        &q.data.lang.code,
                    )}
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
                            {&t("Delete", &q.data.lang.code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/posts"
                                .replace("{lang}", &q.data.lang.code)
                            ,
                            class = "button is-link is-light",
                        ] {
                            {&t("Cancel", &q.data.lang.code)}
                        }
                    }
                }
            }
        }
    }
}

