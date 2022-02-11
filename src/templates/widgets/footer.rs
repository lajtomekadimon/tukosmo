use markup;
use chrono::Datelike;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Footer<'a>(
        t: &'a TranslateI18N,
        data: &'a WebsiteDataDB,
    ) {
        div[
            class = "footer",
        ] {
            div[
                class = "footer-container",
            ] {
                div[
                    class = "footer-credits",
                ] {
                    @t.copyright_w_year_name
                        .replace(
                            "{year}",
                            &chrono::Utc::now().year().to_string()
                        )
                        .replace("{name}", &data.copyright_owner)
                }
            }
        }
    }
}

