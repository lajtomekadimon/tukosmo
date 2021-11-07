use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::widgets::header::Header;
use crate::templates::widgets::navigation::Navigation;
use crate::templates::widgets::sidebar::Sidebar;
use crate::templates::widgets::footer::Footer;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Website<'a, BodyContent: markup::Render>(
        content: BodyContent,
        route: &'a str,
        data: &'a WebsiteDataDB,
        t: &'a TranslateI18N,
    ) {
        div[
            id = "site-languages",
            class = "site-languages",
        ] {
            div[class = "site-languages-content"] {
                h3 {
                    @t.select_a_language
                }

                ul {
                    @for lang in &data.languages {
                        li {
                            a[
                                href = "/{lang}{route}"
                                    .replace("{lang}", &lang.code)
                                    .replace("{route}", route),
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
        div[
            id = "page",
            class = "site",
        ] {
            div[
                class = "site-top",
            ] {
                @Header {
                    data: data,
                }

                @Navigation {
                    data: data,
                }
            }

            div[
                class = "site-content",
            ] {
                div [
                    class = "container",
                ] {
                    div[
                        class = "site-content-inside",
                    ] {
                        main[
                            class = "site-main",
                        ] {
                            @content
                        }

                        @Sidebar {}
                    }
                }
            }

            @Footer {
                t: t,
            }
        }
    }
}

