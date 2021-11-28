use markup;

use crate::i18n::translate_i18n::TranslateI18N;
//use crate::i18n::t_date::t_date;
use crate::templates::admin_layout::AdminLayout;
use crate::templates::widgets::admin_panel::AdminPanel;
use crate::templates::widgets::admin_lang_dropdown::AdminLangDropdown;
//use crate::templates::widgets::admin_pagination::AdminPagination;
use crate::handlers::admin::files::FilesAResponse;


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
                " (WORK IN PROGRESS!)"

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

            /*
            h2[class = "subtitle"] {
                {t.page_n
                    .replace("{n}", &q.page.to_string())}

                " ("
                {(match q.users.iter().len() {
                    1 => t.one_result_of_m,
                    _ => t.n_results_of_m,
                })
                    .replace("{n}", &(q.users.iter().len()).to_string())
                    .replace("{m}", &q.total_results.to_string())
                }
                ")"
            }
            */

            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_files_were_successfully_updated
                }
            }

            div[class = "columns is-multiline"] {
                div[class = "column is-one-quarter-desktop is-half-tablet"] {
                    div[class = "card"] {
                        div[class = "card-image"] {
                            figure[class = "image is-3by2"] {
                                img[
                                    src = "https://unsplash.it/300/200/?random&pic=1",
                                    alt = "",
                                ];
                            }
                            div[class = "card-content is-overlay is-clipped"] {
                                span[class = "tag is-info"] {
                                    "Title"
                                }
                            }
                        }
                        footer[class = "card-footer"] {
                            a[class = "card-footer-item"] {
                                "file_name.txt"
                            }
                        }
                    }
                }

                div[class = "column is-one-quarter-desktop is-half-tablet"] {
                    div[class = "card"] {
                        div[class = "card-image"] {
                            figure[class = "image is-3by2"] {
                                img[
                                    src = "https://unsplash.it/300/200/?random&pic=2",
                                    alt = "",
                                ];
                            }
                            div[class = "card-content is-overlay is-clipped"] {
                                span[class = "tag is-info"] {
                                    "Title"
                                }
                            }
                        }
                        footer[class = "card-footer"] {
                            a[class = "card-footer-item"] {
                                "file_name.txt"
                            }
                        }
                    }
                }

                div[class = "column is-one-quarter-desktop is-half-tablet"] {
                    div[class = "card"] {
                        div[class = "card-image"] {
                            figure[class = "image is-3by2"] {
                                img[
                                    src = "https://unsplash.it/300/200/?random&pic=3",
                                    alt = "",
                                ];
                            }
                            div[class = "card-content is-overlay is-clipped"] {
                                span[class = "tag is-info"] {
                                    "Title"
                                }
                            }
                        }
                        footer[class = "card-footer"] {
                            a[class = "card-footer-item"] {
                                "file_name.txt"
                            }
                        }
                    }
                }
            }

            /*
            @AdminPagination {
                data: &q.data,
                t: t,
                route: "/{lang}/admin/files?p={page}&rpp={rpp}",
                current_page: &q.page,
                total_pages: &q.total_pages,
                results_per_page: &q.results_per_page,
            }
            */
        }
    }
}

