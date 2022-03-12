use markup;

use crate::files::{
    extensions::IMG_EXTS,
    file_size_in_units::file_size_in_units,
};
use crate::handlers::{
    admin::scope_users::edit_get::ra_users_edit_w_id,
    files_get::r_file,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::database::types::{AdminDataDB, FileDB};


markup::define! {
    AdminFileCard<'a>(
        data: &'a AdminDataDB,
        file_data: &'a FileDB,
        t: &'a TranslateI18N,
    ) {

        div[class = "card"] {
            div[class = "card-content has-text-centered"] {
                @if IMG_EXTS.contains(
                    &file_data.ext.as_str(),
                ) {
                    img[
                        src = &r_file(&file_data.name),
                        alt = &file_data.name,
                    ];
                }

                div {
                    strong {
                        @t.size_k_file
                        ": "
                    }
                    {&file_size_in_units(file_data.bytes)}
                }

                div {
                    strong {
                        @t.uploaded_on
                        ": "
                    }
                    {&t_date(&file_data.date, &data.lang.code)}
                }

                div {
                    strong {
                        @t.uploaded_by
                        ": "
                    }

                    a[
                        href = &ra_users_edit_w_id(
                            &data.lang.code,
                            &file_data.author,
                        ),
                    ] {
                        @file_data.author_name
                    }
                }
            }
            div[class = "card-footer"] {
                div[class = "card-footer-item"] {
                    span {
                        a[
                            href = &r_file(&file_data.name),
                            target = "_blank",
                        ] {
                            {&r_file(&file_data.name)}
                        }
                    }
                }
                div[class = "card-footer-item"] {
                    span {
                        a[
                            class = "button is-link is-light",
                            href = &r_file(&file_data.name),
                            download = true,
                        ] {
                            @t.download
                        }
                    }
                }
            }
        }

    }
}

