use markup;

use crate::files::{
    extensions::IMG_EXTS,
    file_route::file_route,
};
use crate::handlers::admin::files_get::{
    AgoFiles,
    ra_files_wu_rpp_p,
};
use crate::handlers::admin::scope_files::{
    new_get::ra_files_new,
    edit_get::ra_files_edit_w_id,
};
use crate::i18n::{
    translate_i18n::TranslateI18N,
    t_date::t_date,
};
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
    widgets::admin_lang_dropdown::AdminLangDropdown,
    widgets::admin_pagination::AdminPagination,
};


markup::define! {
    Files<'a>(
        title: &'a str,
        q: &'a AgoFiles,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        @AdminLayout {
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                    success: success,
                },
                current_page: "files",
                data: &q.data,
                t: t,
            },
        }
    }

    Content<'a>(
        q: &'a AgoFiles,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.files

                @if q.data.languages.iter().len() > 1 {
                    div[class = "is-pulled-right"] {
                        @AdminLangDropdown {
                            routes: &q.routes,
                            data: &q.data,
                        }
                    }
                }

                a[
                    href = ra_files_new(&q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.upload_file
                }
            }

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
            }

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_files_were_successfully_updated
                }
            }

            div[class = "columns is-multiline"] {
                @for file in q.files.iter() {
                    div[
                        class = "column is-one-fifth-desktop is-half-tablet",
                        title = &t.uploaded_by_name_on_date
                            .replace("{name}", &file.author_name)
                            .replace(
                                "{date}",
                                &t_date(&file.date, &q.data.lang.code),
                            ),
                    ] {
                        a[
                            href = ra_files_edit_w_id(
                                &q.data.lang.code,
                                &file.id,
                            ),
                        ] {
                            div[class = "card"] {
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

            @if &q.total_pages > &1 {
                @AdminPagination {
                    data: &q.data,
                    t: t,
                    route: &ra_files_wu_rpp_p(&q.data.lang.code),
                    current_page: &q.page,
                    total_pages: &q.total_pages,
                    results_per_page: &q.results_per_page,
                    buttons: &false,
                }
            }
        }
    }
}

