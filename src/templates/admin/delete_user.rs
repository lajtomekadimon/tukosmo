use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::delete_user::DeleteUserAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    DeleteUser<'a>(
        title: &'a str,
        q: &'a DeleteUserAResponse,
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
                current_page: "delete_user",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a DeleteUserAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Delete user: {name}",
                    &q.data.lang.code
                ).replace("{name}", &q.user_data.name)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/delete_user?id={id}"
                            .replace("{id}", &q.user_data.id.to_string()),
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
                        "Are you sure that you want to delete this user?",
                        &q.data.lang.code,
                    )}
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/delete_user?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &q.user_data.id.to_string()),
            ] {
                input[
                    type = "hidden",
                    name = "id",
                    value = &q.user_data.id,
                ];

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-danger"] {
                            {&t("Delete", &q.data.lang.code)}
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/languages"
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

