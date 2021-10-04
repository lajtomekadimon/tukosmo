use markup;

use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::page::PageWResponse;


markup::define! {
    Page<'a>(
        title: &'a str,
        q: &'a PageWResponse,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {},
                data: &q.data,
            },
        }
    }

    Content() {
        div { "Coming soon!" }
    }
}

