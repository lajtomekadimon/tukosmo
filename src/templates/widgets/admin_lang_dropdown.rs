use markup;

use crate::database::types::{AdminDataDB, RouteDB};


markup::define! {
    AdminLangDropdown<'a>(
        routes: &'a Vec<RouteDB>,
        data: &'a AdminDataDB,
    ) {
        div[
            class = "dropdown",
            id = "dropdown-lang",
        ] {
            div[class = "dropdown-trigger"] {
                button[
                    class = "button",
                    id = "dropdown-lang-button",
                    "aria-haspopup" = "true",
                    "aria-controls" = "dropdown-menu-lang",
                ] {
                    span {
                        @data.lang.name
                    }
                    span[class = "icon is-small"] {
                        i[
                            class = "eos-icons notranslate",
                            translate = "no",
                        ] {
                            "arrow_drop_down"
                        }
                    }
                }
            }
            div[
                class = "dropdown-menu",
                id = "dropdown-menu-lang",
                role = "menu",
            ] {
                div[class = "dropdown-content"] {
                    @for route in routes.iter() {
                        a[
                            href = &route.route,
                            class = if &route.lang.code != &data.lang.code {
                                "dropdown-item"
                            } else {
                                "dropdown-item is-active"
                            },
                        ] {
                            @route.lang.name
                            @if data.lang.code != route.lang.code {
                                " ("
                                @route.lang.original_name
                                ")"
                            }
                        }
                    }
                }
            }
        }
    }
}

