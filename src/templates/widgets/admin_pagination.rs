use markup;

use crate::i18n::t::t;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminPagination<'a>(
        data: &'a AdminDataDB,
        route: &'a str,
        current_page: &'a i64,
        total_pages: &'a i64,
        results_per_page: &'a i64,
    ) {
        nav[
            class = "pagination is-centered pt-5",
            role = "navigation",
            "aria-label" = "pagination",
        ] {
            @if **current_page == 1 {
                button[
                    class = "pagination-previous",
                    disabled = true,
                ] {
                    {&t("Previous [page]", &data.lang.code)}
                }
            } else {
                a[
                    class = "pagination-previous",
                    href = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{page}", &(**current_page - 1).to_string()),
                ] {
                    {&t("Previous [page]", &data.lang.code)}
                }
            }

            @if current_page == total_pages {
                button[
                    class = "pagination-next",
                    disabled = true,
                ] {
                    {&t("Next [page]", &data.lang.code)}
                }
            } else {
                a[
                    class = "pagination-next",
                    href = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{page}", &(**current_page + 1).to_string()),
                ] {
                    {&t("Next [page]", &data.lang.code)}
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

