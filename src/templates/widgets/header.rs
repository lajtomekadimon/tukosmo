use markup;

use crate::database::types::WebsiteDataDB;
use crate::handlers::website::home::rw_home;


markup::define! {
    Header<'a>(
        data: &'a WebsiteDataDB,
    ) {
        header[class = "site-header"] {
            div[class = "container"] {
                div[class = "site-branding"] {
                    h1[class = "site-title"] {
                        a[href = rw_home(&data.lang.code)] {
                            @data.website_title
                        }
                    }

                    p[class = "site-description"] {
                        @data.website_subtitle
                    }
                }
            }
        }
    }
}

