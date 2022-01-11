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
            class = "site-footer",
        ] {
            div[
                class = "container",
            ] {
                div[
                    class = "site-credits",
                ] {
                    @t.copyright_year_name
                    .replace("{year}", &chrono::Utc::now().year().to_string())
                    .replace("{name}", &data.copyright_owner)
                }
            }
        }
    }
}

