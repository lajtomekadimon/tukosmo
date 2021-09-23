use markup;

use crate::i18n::t::t;
use crate::database::types::WebsiteDataDB;
use chrono::Datelike;


markup::define! {
    Footer<'a>(
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
                    {&t(
                        "{name} © {year} [copyright]",
                        &data.lang.code
                    )
                    .replace("{name}", "Lajto")  // TODO: This shouldn't be
                                                 // the website's name, but
                                                 // the company/author/owner's
                    .replace("{year}", &chrono::Utc::now().year().to_string())}
                }
            }
        }
    }
}

