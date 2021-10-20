use markup;

use crate::i18n::t::t;
use crate::database::types::WebsiteDataDB;


markup::define! {
    BlogPagination<'a>(
        data: &'a WebsiteDataDB,
        route: &'a str,
        current_page: &'a i64,
        total_pages: &'a i64,
        results_per_page: &'a i64,
    ) {
        nav[
            class = "blog-pagination",
            role = "navigation",
            "aria-label" = "pagination",
        ] {
            @if **current_page == 1 {
                button[
                    class = "blog-pagination-previous",
                    disabled = true,
                ] {
                    {&t("Previous [page]", &data.lang.code)}
                }
            } else {
                a[
                    class = "blog-pagination-previous",
                    href = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{rpp}", &results_per_page.to_string())
                        .replace("{page}", &(**current_page - 1).to_string()),
                ] {
                    {&t("Previous [page]", &data.lang.code)}
                }
            }

            @if current_page == total_pages {
                button[
                    class = "blog-pagination-next",
                    disabled = true,
                ] {
                    {&t("Next [page]", &data.lang.code)}
                }
            } else {
                a[
                    class = "blog-pagination-next",
                    href = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{rpp}", &results_per_page.to_string())
                        .replace("{page}", &(**current_page + 1).to_string()),
                ] {
                    {&t("Next [page]", &data.lang.code)}
                }
            }

            ul[class = "blog-pagination-list"] {
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
                                    class = "blog-pagination-ellipsis",
                                ] {
                                    "…"
                                }
                            }
                        }

                        li {
                            a[
                                class = if p == **current_page {
                                    "blog-pagination-link is-current"
                                } else {
                                    "blog-pagination-link"
                                },
                                href = route
                                    .replace("{lang}", &data.lang.code)
                                    .replace("{rpp}", &results_per_page.to_string())
                                    .replace("{page}", &p.to_string())
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
                                    class = "blog-pagination-ellipsis",
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

