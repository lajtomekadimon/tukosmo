use markup;

use crate::handlers::admin::{
    scope_json::files_selector_get::ra_json_files_selector_w_rpp_p,
};
use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::FileDB;


markup::define! {
    AdminFileSelector<'a>(
        t: &'a TranslateI18N,
        name: &'a str,
        current_file: &'a Option<FileDB>,
    ) {

        div[class = "file has-name is-boxed"] {
            label[
                id = "file-selector-js",
                class = "file-label",
                "data-url" = ra_json_files_selector_w_rpp_p(
                    &t.lang_code,
                    &12,
                    &1
                ),
            ] {
                input[
                    id = "file-input",
                    type = "hidden",
                    name = name,
                    value = if let Some(f) = current_file {
                        f.id
                    } else {
                        0
                    },
                ];
                span[class = "file-cta"] {
                    span[class = "file-icon"] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] { "cloud_upload" }
                    }
                    span[class = "file-label"] {
                        @t.choose_a_file
                    }
                }
                span[
                    id = "file-name",
                    class = "file-name",
                    style = if let Some(_f) = current_file {
                        "display: none;"
                    } else {
                        ""
                    },
                ] {
                    @t.no_file_uploaded
                }
                span[
                    id = "file-imgcard",
                    class = "file-name",
                    style = if let Some(_f) = current_file {
                        "height: auto;"
                    } else {
                        "height: auto; display: none;"
                    },
                ] {
                    img[
                        id = "file-img",
                        src = if let Some(f) = current_file {
                            &f.route
                        } else {
                            ""
                        },
                        alt = if let Some(f) = current_file {
                            &f.name
                        } else {
                            ""
                        },
                    ];
                }
            }
        }

        button[
            id = "file-selector-cancel",
            class = if let Some(_f) = current_file {
                "button is-danger is-light \
                has-text-weight-normal"
            } else {
                "button is-danger is-light \
                has-text-weight-normal is-hidden"
            },
            type = "button",
        ] {
            @t.remove_featured_image
        }

    }
}

