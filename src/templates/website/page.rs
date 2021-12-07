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
            routes: &q.routes,
            og_title: "Page (WIP!)",
            og_description: "",
            og_image: "",
            og_article: &None,
            content: Website {
                content: Content {},
                routes: &q.routes,
                data: &q.data,
                t: t,
            },
        }
    }

    Content() {
        div { "Coming soon!" }
    }
}

