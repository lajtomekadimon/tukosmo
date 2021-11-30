use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::handlers::admin::delete_file::DeleteFileAResponse;
use crate::i18n::t_error::ErrorDB;


markup::define! {
    DeleteFile<'a>(
        title: &'a str,
        q: &'a DeleteFileAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    error: error,
                },
                current_page: "delete_file",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a DeleteFileAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.delete_file

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        routes: &q.routes,
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
                    @t.are_you_sure_that_you_want_to_delete_this_file
                }
            }

            form[
                method = "post",
                action = "/{lang}/admin/delete_file?id={id}"
                    .replace("{lang}", &q.data.lang.code)
                    .replace("{id}", &q.file_data.id.to_string()),
            ] {
                input[
                    type = "hidden",
                    name = "csrf_token",
                    value = &q.csrf_token,
                ];

                input[
                    type = "hidden",
                    name = "id",
                    value = &q.file_data.id,
                ];

                div[class = "field is-grouped"] {
                    div[class = "control"] {
                        button[class = "button is-danger"] {
                            @t.delete
                        }
                    }
                    div[class = "control"] {
                        a[
                            href = "/{lang}/admin/files"
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

