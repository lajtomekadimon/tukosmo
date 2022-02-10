use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Pagination<'a>(
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
        route: &'a str,
        current_page: &'a i64,
        total_pages: &'a i64,
        results_per_page: &'a i64,
    ) {
        nav[
            class = "pagination",
            role = "navigation",
            "aria-label" = "pagination",
        ] {
            @if **current_page == 1 {
                button[
                    class = "pagination-previous",
                    disabled = true,
                ] {
                    @t.previous
                }
            } else {
                a[
                    class = "pagination-previous",
                    href = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{rpp}", &results_per_page.to_string())
                        .replace("{p}", &(**current_page - 1).to_string()),
                ] {
                    @t.previous
                }
            }

            @if current_page == total_pages {
                button[
                    class = "pagination-next",
                    disabled = true,
                ] {
                    @t.next
                }
            } else {
                a[
                    class = "pagination-next",
                    href = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{rpp}", &results_per_page.to_string())
                        .replace("{p}", &(**current_page + 1).to_string()),
                ] {
                    @t.next
                }
            }

            ul[class = "pagination-list"] {
                @for p in 1..(**total_pages + 1) {

                    // TODO: Optimize using a vector of just 1~5 elements
                    @if p == 1 ||
                        p == (**current_page - 1) ||
                        p == **current_page ||
                        p == (**current_page + 1) ||
                        p == **total_pages
                    {

                        @if p == (**current_page - 1) && **current_page > 3 {
                            li {
                                span[
                                    class = "pagination-ellipsis",
                                ] {
                                    "…"
                                }
                            }
                        }

                        li {
                            a[
                                class = if p == **current_page {
                                    "pagination-link is-current"
                                } else {
                                    "pagination-link"
                                },
                                href = route
                                    .replace("{lang}", &data.lang.code)
                                    .replace("{rpp}", &results_per_page.to_string())
                                    .replace("{p}", &p.to_string())
                                    .replace(
                                        "{rpp}",
                                        &results_per_page.to_string()
                                    ),
                            ] {
                                @p.to_string()
                            }
                        }
                        
                        @if p == (**current_page + 1) &&
                            **current_page < (**total_pages - 2)
                        {
                            li {
                                span[
                                    class = "pagination-ellipsis",
                                ] {
                                    "…"
                                }
                            }
                        }

                    }

                }
            }
        }
    }
}

