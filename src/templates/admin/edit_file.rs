use actix_web::web::Form as ActixForm;
use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::templates::widgets::admin_file_card::AdminFileCard;
use crate::handlers::admin::edit_file::EditFileAResponse;
use crate::handlers::admin::edit_file_post::FormData;
use crate::database::error_codes as ec;
use crate::i18n::t_error::ErrorDB;
use crate::handlers::admin::edit_file::ra_edit_file_w_id;
use crate::handlers::admin::files::ra_files;
use crate::handlers::admin::delete_file::ra_delete_file_w_id;


markup::define! {
    EditFile<'a>(
        title: &'a str,
        q: &'a EditFileAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
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
                    form: form,
                },
                current_page: "edit_file",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a EditFileAResponse,
        t: &'a TranslateI18N,
        error: &'a Option<ErrorDB>,
        form: &'a Option<ActixForm<FormData>>,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.edit_file

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
                action = ra_edit_file_w_id(
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

                    // Edit file
                    div[class = "column"] {
                        div[class = "field"] {
                            label[class = "label"] {
                                @t.filename
                            }
                            div[class = "control"] {
                                input[
                                    class = if let Some(e) = error {
                                        if e.code == ec::WRONG_FILENAME {
                                            "input is-danger"
                                        } else {
                                            "input"
                                        }
                                    } else {
                                        "input"
                                    },
                                    type = "text",
                                    name = "filename",
                                    value = if let Some(f) = form {
                                        &f.filename
                                    } else { &q.file_data.name },
                                ];
                            }
                        }

                        div[class = "field is-grouped"] {
                            div[class = "control"] {
                                button[class = "button is-link"] {
                                    @t.submit
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

                            div[class = "control"] {
                                a[
                                    href = ra_delete_file_w_id(
                                        &q.data.lang.code,
                                        &q.file_data.id,
                                    ),
                                    class = "button is-danger \
                                             has-text-weight-normal mr-4",
                                ] {
                                    @t.delete
                                }
                            }
                        }
                    }

                }
            }
        }
    }
}

