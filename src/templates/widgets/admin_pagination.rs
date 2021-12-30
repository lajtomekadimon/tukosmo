use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::AdminDataDB;


markup::define! {
    AdminPagination<'a>(
        data: &'a AdminDataDB,
        t: &'a TranslateI18N,
        route: &'a str,
        current_page: &'a i64,
        total_pages: &'a i64,
        results_per_page: &'a i64,
        buttons: &'a bool,
    ) {
        nav[
            class = "pagination is-centered",
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
            } else if **buttons {
                button[
                    class = "file-selector-page button pagination-previous",
                    "data-route" = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{rpp}", &results_per_page.to_string())
                        .replace("{p}", &(**current_page - 1).to_string()),
                    "data-rpp" = &results_per_page.to_string(),
                    "data-p" = &(**current_page - 1).to_string(),
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
            } else if **buttons {
                button[
                    class = "file-selector-page button pagination-next",
                    "data-route" = route
                        .replace("{lang}", &data.lang.code)
                        .replace("{rpp}", &results_per_page.to_string())
                        .replace("{p}", &(**current_page + 1).to_string()),
                    "data-rpp" = &results_per_page.to_string(),
                    "data-p" = &(**current_page + 1).to_string(),
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
                            @if **buttons {
                                button[
                                    class = if p == **current_page {
                                        "file-selector-page button pagination-link is-current"
                                    } else {
                                        "file-selector-page button pagination-link"
                                    },
                                    "data-route" = route
                                        .replace("{lang}", &data.lang.code)
                                        .replace(
                                            "{rpp}",
                                            &results_per_page.to_string(),
                                        )
                                        .replace("{p}", &p.to_string()),
                                    "data-rpp" = &results_per_page.to_string(),
                                    "data-p" = &p.to_string(),
                                ] {
                                    @p.to_string()
                                }
                            } else {
                                a[
                                    class = if p == **current_page {
                                        "pagination-link is-current"
                                    } else {
                                        "pagination-link"
                                    },
                                    href = route
                                        .replace("{lang}", &data.lang.code)
                                        .replace(
                                            "{rpp}",
                                            &results_per_page.to_string(),
                                        )
                                        .replace("{p}", &p.to_string()),
                                ] {
                                    @p.to_string()
                                }
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

