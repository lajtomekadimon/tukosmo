use markup;

use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::database::types::WebsiteDataDB;


markup::define! {
    Page<'a>(
        title: &'a str,
        data: &'a WebsiteDataDB,
    ) {
        @WebsiteLayout {
            title: title,
            data: &data,
            content: Website {
                content: Content {},
                data: data,
            },
        }
    }

    Content() {
        div { "Coming soon!" }
    }
}

