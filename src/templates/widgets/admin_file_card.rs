use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::{AdminDataDB, FileDB};
use crate::i18n::t_date::t_date;
use crate::files::extensions::IMG_EXTS;


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
                    img[
                        src = "/files/{name}".replace(
                            "{name}",
                            &file_data.name,
                        ),
                    ];
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
                        href = "/{lang}/admin/edit_user\
                                ?id={id}"
                            .replace(
                                "{lang}",
                                &data.lang.code,
                            )
                            .replace(
                                "{id}",
                                &file_data.author.to_string()
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
                            href = "/files/{name}".replace(
                                "{name}",
                                &file_data.name,
                            ),
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
                            href = "/files/{name}".replace(
                                "{name}",
                                &file_data.name,
                            ),
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

