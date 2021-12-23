use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::templates::widgets::admin_file_card::AdminFileCard;
use crate::handlers::admin::delete_file::DeleteFileAResponse;
use crate::i18n::t_error::ErrorDB;
use crate::handlers::admin::delete_file::ra_delete_file_w_id;
use crate::handlers::admin::files::ra_files;


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
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.delete_file

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
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

            form[
                method = "post",
                action = ra_delete_file_w_id(
                    &q.data.lang.code,
                    &q.file_data.id,
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
                    value = &q.file_data.id,
                ];

                div[class = "columns"] {

                    // File
                    div[class = "column"] {
                        @AdminFileCard {
                            data: &q.data,
                            file_data: &q.file_data,
                            t: t,
                        }
                    }

                    // Delete file
                    div[class = "column"] {
                        div[class = "content"] {
                            p {
                                @t.are_you_sure_that_you_want_to_delete_this_file
                            }
                        }
                        div[class = "field is-grouped"] {
                            div[class = "control"] {
                                button[class = "button is-danger"] {
                                    @t.delete
                                }
                            }
                            div[class = "control"] {
                                a[
                                    href = ra_files(&q.data.lang.code),
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
    }
}

