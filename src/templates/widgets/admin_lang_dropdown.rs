use markup;

use crate::database::s_languages::s_languages;


markup::define! {
    AdminLangDropdown<'a>(
        lang_code: &'a str,
        route: &'a str,
    ) {
        div[class = "dropdown is-hoverable"] {
            div[class = "dropdown-trigger"] {
                button[
                    class = "button",
                    "aria-haspopup" = "true",
                    "aria-controls" = "dropdown-menu",
                ] {
                    span {
                        @for lang in s_languages(lang_code.to_string()) {
                            @if &lang.code == lang_code {
                                @lang.name
                            }
                        }
                    }
                    span[class = "icon is-small"] {
                        i[
                            class = "fas fa-angle-down",
                            "aria-hidden" = "true",
                        ] {}
                    }
                }
            }
            div[
                class = "dropdown-menu",
                id = "dropdown-menu",
                role = "menu",
            ] {
                div[class = "dropdown-content"] {
                    @for lang in s_languages(lang_code.to_string()) {
                        a[
                            href = "/{lang}{route}"
                                .replace("{lang}", &lang.code)
                                .replace("{route}", route),
                            class = if &lang.code != lang_code {
                                "dropdown-item"
                            } else {
                                "dropdown-item is-active"
                            },
                        ] {
                            @lang.name
                        }
                    }
                }
            }
        }
    }
}

