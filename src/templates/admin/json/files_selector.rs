use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::widgets::admin_pagination::AdminPagination;
use crate::handlers::admin::json::files_selector::{
    FilesSelectorAJResponse,
    raj_files_selector_wu_rpp_p,
};
use crate::files::extensions::IMG_EXTS;
use crate::files::file_route::file_route;


markup::define! {
    FilesSelector<'a>(
        q: &'a FilesSelectorAJResponse,
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

            section[class = "modal-card-body"] {
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
                        id = "file-selector-cancel",
                        class = "button is-danger is-light is-pulled-right \
                                 has-text-weight-normal mr-4",
                    ] {
                        @t.remove_featured_image
                    }
                }

                div[class = "columns is-multiline"] {
                    @for file in q.files.iter() {
                        div[
                            class = "column is-4 is-half-tablet",
                            title = &t.uploaded_by_name_on_date
                                .replace("{name}", &file.author_name)
                                .replace(
                                    "{date}",
                                    &t_date(&file.date, &q.data.lang.code),
                                ),
                        ] {
                            button[
                                id = "select-file-{id}".replace("{id}", &file.id.to_string()),
                                class = "select-file nostyle-button mb-4",
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
                                                    src = file_route(&file.name),
                                                    alt = &file.name,
                                                ];
                                            }
                                        }
                                        div[class = "card-content is-overlay is-clipped"] {
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
                        route: &raj_files_selector_wu_rpp_p(&q.data.lang.code),
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

