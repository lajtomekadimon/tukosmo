use markup;

use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::templates::widgets::post::Post;
use crate::database::types::{WebsiteDataDB, PostDB};


markup::define! {
    BlogPost<'a>(
        title: &'a str,
        data: &'a WebsiteDataDB,
        post: &'a PostDB,
    ) {
        @WebsiteLayout {
            title: title,
            data: &data,
            content: Website {
                content: Content {
                    data: data,
                    post: post,
                },
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a WebsiteDataDB,
        post: &'a PostDB,
    ) {
        @Post {
            data: data,
            post: post,
        }
    }
}

