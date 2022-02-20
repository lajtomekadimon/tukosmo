use markup;

use crate::files::{
    extensions::IMG_EXTS,
    file_route::file_route,
};
use crate::handlers::admin::{
    scope_json::files_selector_get::{
        AgoJsonFilesSelector,
        ra_json_files_selector_w_rpp_p,
        ra_json_files_selector_wu_rpp_p,
    },
    scope_json::upload_file_post::ra_json_upload_file,
    scope_json::edit_file_post::ra_json_edit_file,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::widgets::admin_pagination::AdminPagination;


markup::define! {
    FilesSelector<'a>(
        q: &'a AgoJsonFilesSelector,
        t: &'a TranslateI18N,
    ) {

        div[class = "modal-background file-selector-close"] {}

        div[class = "modal-card"] {
            header[class = "modal-card-head"] {
                p[class = "modal-card-title"] {
                    @t.choose_a_file
                }
                button[
                    class = "delete file-selector-close",
                ] {}
            }

            section[
                id = "file-selector-uploading",
                class = "modal-card-body is-hidden",
            ] {
                h2[class = "subtitle"] {
                    @t.upload_file

                    button[
                        id = "file-selector-select-button",
                        class = "button is-link is-pulled-right \
                                 has-text-weight-normal mr-4",
                        type = "button",
                        "data-url" = ra_json_files_selector_w_rpp_p(
                            &t.lang_code,
                            &12,
                            &1,
                        ),
                    ] {
                        @t.choose_a_file
                    }
                }

                div[class = "field"] {
                    label[class = "label"] {
                        @t.file
                    }
                    div[
                        id = "file-upload",
                        class = "file has-name",
                    ] {
                        label[class = "file-label"] {
                            input[
                                id = "file-upload-file",
                                "data-url" = ra_json_upload_file(
                                    &q.data.lang.code,
                                ),
                                class = "file-input",
                                type = "file",
                                name = "resume",
                            ];
                            span[class = "file-cta"] {
                                span[class = "file-icon"] {
                                    i[
                                        class = "eos-icons notranslate",
                                        translate = "no"
                                    ] { "cloud_upload" }
                                }
                                span[class = "file-label"] {
                                    @t.choose_a_file
                                }
                            }
                            span[class = "file-name"] {
                                @t.no_file_uploaded
                            }
                        }
                    }
                }

                progress[
                    id = "file-upload-progress",
                    class = "progress is-large is-info",
                    style = "display: none;",
                ] {}

                div[
                    id = "file-upload-notif-success",
                    class = "notification is-success",
                    style = "display: none;",
                ] {
                    button[class = "delete"] {}
                    @t.your_file_has_been_successfully_uploaded
                }

                div[
                    id = "file-upload-form",
                    style = "display: none;",
                ] {
                    input[
                        type = "hidden",
                        name = "csrf_token",
                        value = &q.csrf_token,
                    ];

                    input[
                        type = "hidden",
                        name = "id",
                        value = "",
                    ];

                    div[class = "field"] {
                        label[class = "label"] {
                            @t.file_title
                        }
                        div[class = "control"] {
                            input[
                                class = "input",
                                type = "text",
                                name = "file_title",
                                value = "",
                            ];
                        }
                    }

                    div[class = "field is-grouped"] {
                        div[class = "control"] {
                            button[
                                id = "file-selector-upload-change",
                                class = "button is-link",
                                type = "button",
                                "data-formurl" = ra_json_edit_file(
                                    &t.lang_code,
                                ),
                                "data-url" = ra_json_files_selector_w_rpp_p(
                                    &t.lang_code,
                                    &12,
                                    &1,
                                ),
                            ] {
                                @t.submit
                            }
                        }
                        div[class = "control"] {
                            button[
                                id = "file-selector-upload-nochange",
                                class = "button is-link is-light",
                                type = "button",
                                "data-url" = ra_json_files_selector_w_rpp_p(
                                    &t.lang_code,
                                    &12,
                                    &1,
                                ),
                            ] {
                                @t.cancel
                            }
                        }
                    }
                }
            }

            section[
                id = "file-selector-selection",
                class = "modal-card-body",
            ] {
                h2[class = "subtitle"] {
                    {t.page_n
                        .replace("{n}", &q.page.to_string())}

                    " ("
                    {(match q.files.iter().len() {
                        1 => t.one_result_of_m,
                        _ => t.n_results_of_m,
                    })
                        .replace("{n}", &(q.files.iter().len()).to_string())
                        .replace("{m}", &q.total_results.to_string())
                    }
                    ")"

                    button[
                        id = "file-selector-upload-button",
                        class = "button is-link is-pulled-right \
                                 has-text-weight-normal mr-4",
                        type = "button",
                    ] {
                        @t.upload_file
                    }
                }

                div[class = "columns is-multiline"] {
                    @for file in q.files.iter() {
                        div[
                            class = "column is-4 is-half-tablet",
                        ] {
                            button[
                                id = "select-file-{id}"
                                    .replace("{id}", &file.id.to_string()),
                                class =
                                    "select-file nostyle-button mb-4 width100",
                                type = "button",
                                title = &t.uploaded_by_name_on_date
                                    .replace("{name}", &file.author_name)
                                    .replace(
                                        "{date}",
                                        &t_date(&file.date, &q.data.lang.code),
                                    ),
                                "data-id" = &file.id,
                                "data-name" = &file.name,
                                "data-route" = &file.route,
                            ] {
                                div[class = "card m-0"] {
                                    div[class = "card-image"] {
                                        figure[class = "image is-3by2"] {
                                            @if IMG_EXTS.contains(
                                                &file.ext.as_str()
                                            ) {
                                                img[
                                                    src = file_route(
                                                        &file.name
                                                    ),
                                                    alt = &file.name,
                                                ];
                                            }
                                        }
                                        div[
                                            class = "card-content is-overlay \
                                                     is-clipped",
                                        ] {
                                            span[class = "tag is-link"] {
                                                @file.ext.to_uppercase()
                                            }
                                        }
                                    }
                                    footer[class = "card-footer"] {
                                        span[class = "card-footer-item"] {
                                            @file.name
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            @if &q.total_pages > &1 {
                footer[class = "modal-card-foot"] {
                    @AdminPagination {
                        data: &q.data,
                        t: t,
                        route: &ra_json_files_selector_wu_rpp_p(
                            &q.data.lang.code
                        ),
                        current_page: &q.page,
                        total_pages: &q.total_pages,
                        results_per_page: &q.results_per_page,
                        buttons: &true,
                    }

                    /*
                    div{
                        button[class = "button is-link"] {
                            @t.upload_file
                        }

                        button[class = "button is-pulled-right"] {
                            @t.cancel
                        }
                    }
                    */
                }
            }
        }

    }
}

