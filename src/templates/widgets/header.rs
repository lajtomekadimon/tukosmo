use markup;

use crate::handlers::website::home_get::rw_home;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Header<'a>(
        data: &'a WebsiteDataDB,
    ) {
        header[class = "header"] {
            div[class = "header-container"] {
                div[class = "header-branding"] {
                    h1[class = "header-title"] {
                        a[href = rw_home(&data.lang.code)] {
                            @data.website_title
                        }
                    }

                    p[class = "header-description"] {
                        @data.website_subtitle
                    }
                }
            }
        }
    }
}

