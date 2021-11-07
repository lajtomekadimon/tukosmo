use markup;

use crate::database::types::AdminDataDB;


markup::define! {
    AdminLangDropdown<'a>(
        route: &'a str,
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
                        i[class = "eos-icons"] {
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
                    @for lang in &data.languages {
                        a[
                            href = "/{lang}{route}"
                                .replace("{lang}", &lang.code)
                                .replace("{route}", route),
                            class = if &lang.code != &data.lang.code {
                                "dropdown-item"
                            } else {
                                "dropdown-item is-active"
                            },
                        ] {
                            @lang.name
                            @if data.lang.code != lang.code {
                                " ("
                                @lang.original_name
                                ")"
                            }
                        }
                    }
                }
            }
        }
    }
}

