use markup;

use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::handlers::website::page::PageWResponse;


markup::define! {
    Page<'a>(
        title: &'a str,
        q: &'a PageWResponse,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            title: title,
            data: &q.data,
            content: Website {
                content: Content {},
                route: "/page/{title}",
                data: &q.data,
                t: t,
            },
        }
    }

    Content() {
        div { "Coming soon!" }
    }
}

