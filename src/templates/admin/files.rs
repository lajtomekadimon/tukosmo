use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
use crate::templates::widgets::admin_pagination::AdminPagination;
use crate::handlers::admin::files::FilesAResponse;
use crate::files::extensions::IMG_EXTS;


markup::define! {
    Files<'a>(
        title: &'a str,
        q: &'a FilesAResponse,
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
        q: &'a FilesAResponse,
        t: &'a TranslateI18N,
        success: &'a bool,
    ) {
        div[class = "box is-marginless"] {
            h1[class = "title"] {
                @t.files

                div[class = "is-pulled-right"] {
                    @AdminLangDropdown {
                        routes: &q.routes,
                        data: &q.data,
                    }
                }

                a[
                    href = "/{lang}/admin/upload_file"
                        .replace("{lang}", &q.data.lang.code),
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
                        class = "column is-one-quarter-desktop is-half-tablet",
                        title = &t.uploaded_by_name_on_date
                            .replace("{name}", &file.author_name)
                            .replace(
                                "{date}",
                                &t_date(&file.date, &q.data.lang.code),
                            ),
                    ] {
                        div[class = "card"] {
                            div[class = "card-image"] {
                                figure[class = "image is-3by2"] {
                                    @if IMG_EXTS.contains(&file.ext.as_str()) {
                                        img[
                                            src = "/files/{filename}".replace(
                                                "{filename}",
                                                &file.name,
                                            ),
                                            alt = &file.name,
                                        ];
                                    }
                                }
                            }
                            footer[class = "card-footer"] {
                                a[
                                    class = "card-footer-item",
                                    href = "/{lang}/admin/edit_file?id={id}"
                                        .replace("{lang}", &q.data.lang.code)
                                        .replace(
                                            "{id}",
                                            &(file.id).to_string(),
                                        ),
                                ] {
                                    @file.name
                                }
                            }
                        }
                    }
                }
            }

            @AdminPagination {
                data: &q.data,
                t: t,
                route: "/{lang}/admin/files?p={page}&rpp={rpp}",
                current_page: &q.page,
                total_pages: &q.total_pages,
                results_per_page: &q.results_per_page,
            }
        }
    }
}

