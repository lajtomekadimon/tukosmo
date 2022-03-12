use markup;

use crate::handlers::admin::pages_get::AgoPages;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    admin_layout::AdminLayout,
    widgets::admin_panel::AdminPanel,
};


markup::define! {
    Pages<'a>(
        domain: &'a str,
        codename: &'a str,
        title: &'a str,
        q: &'a AgoPages,
        t: &'a TranslateI18N,
    ) {
        @AdminLayout {
            domain: domain,
            codename: codename,
            title: title,
            data: &q.data,
            routes: &q.routes,
            content: AdminPanel {
                content: Content {
                    q: q,
                    t: t,
                },
                codename: codename,
                current_page: "pages",
                data: &q.data,
                t: t,
                routes: &q.routes,
            },
        }
    }

    Content<'a>(
        q: &'a AgoPages,
        t: &'a TranslateI18N,
    ) {
        div[class = "box is-marginless mb-6"] {
            h1[class = "title"] {
                @t.pages

                /*
                a[
                    href = ra_pages_new(&q.data.lang.code),
                    class = "button is-link is-pulled-right \
                             has-text-weight-normal mr-4",
                ] {
                    @t.new_page
                }
                */
            }

            h2[class = "subtitle"] {
                @t.page_n
                    .replace("{n}", &q.page.to_string())

                /*
                " ("
                {(match q.pages.iter().len() {
                    1 => t.one_result_of_m,
                    _ => t.n_results_of_m,
                })
                    .replace("{n}", &(q.pages.iter().len()).to_string())
                    .replace("{m}", &q.total_results.to_string())
                }
                ")"
                */
            }

            /*
            @if **success {
                div[
                    class = "notification is-success",
                ] {
                    button[class = "delete"] {}
                    @t.your_website_pages_were_successfully_updated
                }
            }
            */

            table[
                class = "table is-bordered is-hoverable is-fullwidth",
            ] {
                thead {
                    tr {
                        th {
                            @t.title
                        }
                        th {
                            @t.status
                        }
                        th {
                            @t.published
                        }
                        th {
                            @t.author
                        }
                    }
                }
                tbody {
                    tr {
                        td { "-" }
                        td { "-" }
                        td { "-" }
                        td { "-" }
                    }
                }
            }

            /*
            @if &q.total_pages > &1 {
                @AdminPagination {
                    data: &q.data,
                    t: t,
                    route: &ra_pages_w_f_wu_rpp_p(
                        &q.data.lang.code,
                        if q.filter == "drafts" {
                            "drafts"
                        } else if q.filter == "published" {
                            "published"
                        } else if q.filter == "untranslated" {
                            "untranslated"
                        } else if q.filter == "deleted" {
                            "deleted"
                        } else {
                            "all"
                        },
                    ),
                    current_page: &q.page,
                    total_pages: &q.total_pages,
                    results_per_page: &q.results_per_page,
                    buttons: &false,
                }
            }
            */
        }
    }
}

