use markup;

use crate::i18n::t::t;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::delete_language::DeleteLanguageAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    DeleteLanguage<'a>(
        title: &'a str,
        q: &'a DeleteLanguageAResponse,
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
                current_page: "delete_language",
                data: &q.data,
            },
        }
    }

    Content<'a>(
        q: &'a DeleteLanguageAResponse,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                {&t(
                    "Delete language: {name}",
                    &q.data.lang.code
                ).replace("{name}", &q.lang.name)}

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        route: &"/admin/delete_language?id={id}"
                            .replace("{id}", &q.lang.id.to_string()),
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
                        "Are you sure that you want to delete this language?",
                        &q.data.lang.code,
                    )}
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/delete_language?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &q.lang.id.to_string()),
            ] {
                input[
                    type = "hidden",
                    name = "id",
                    value = &q.lang.id,
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

