use markup;

use crate::files::{
    extensions::IMG_EXTS,
    file_route::file_route,
};
use crate::handlers::admin::scope_users::edit_get::ra_users_edit_w_id;
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
            div[class = "card-content"] {
                @if IMG_EXTS.contains(
                    &file_data.ext.as_str(),
                ) {
                    img[src = &file_route(&file_data.name)];
                }

                // TODO: File size

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
                            href = &file_route(&file_data.name),
                            target = "_blank",
                        ] {
                            "/files/"
                            @file_data.name
                        }
                    }
                }
                div[class = "card-footer-item"] {
                    span {
                        a[
                            class = "button is-link is-light",
                            href = &file_route(&file_data.name),
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

