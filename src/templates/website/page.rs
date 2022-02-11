use markup;

use crate::handlers::website::page_get::WgoPage;
use crate::i18n::translate_i18n::TranslateI18N;
use crate::templates::{
    website_layout::WebsiteLayout,
    widgets::site::Site,
};


markup::define! {
    Page<'a>(
        domain: &'a str,
        title: &'a str,
        q: &'a WgoPage,
        t: &'a TranslateI18N,
    ) {
        @WebsiteLayout {
            domain: domain,
            title: title,
            data: &q.data,
            routes: &q.routes,
            og_title: "Page (WIP!)",
            og_description: "",
            og_image: "",
            og_article: &None,
            content: Site {
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

