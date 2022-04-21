use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::WebsiteDataDB;
use crate::handlers::website::blog_get::rw_blog;
use crate::templates::widgets::icons;


markup::define! {
    Navigation<'a>(
        t: &'a TranslateI18N,
        data: &'a WebsiteDataDB,
    ) {
        nav[class = "navigation"] {
            div[class = "navigation-container"] {
                button[
                    id = "navigation-burger",
                    class = "navigation-burger",
                ] {
                    span[class = "navigation-burger-text"] {
                        @t.menu
                    }

                    span[class = "navigation-burger-icon"] {
                        @icons::Menu {}
                    }
                }

                ul[
                    id = "navigation-ul",
                    class = "navigation-ul",
                ] {
                    li[class = "navigation-li"] {
                        a[href = rw_blog(&data.lang.code)] {
                            @t.blog
                        }
                    }

                    @if data.languages.iter().len() > 1 {
                        li[class = "navigation-li"] {
                            button[id = "button-select-language"] {
                                @icons::Language {}
                                {data.lang.code.to_uppercase()}
                            }
                        }
                    }
                }
            }
        }
    }
}

