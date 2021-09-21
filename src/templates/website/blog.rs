use markup;

use crate::templates::website_layout::WebsiteLayout;
use crate::templates::widgets::website::Website;
use crate::templates::widgets::post_list::PostList;
use crate::database::types::{WebsiteDataDB, PostDB};


markup::define! {
    Blog<'a>(
        title: &'a str,
        data: &'a WebsiteDataDB,
        posts: &'a Vec<PostDB>,
    ) {
        @WebsiteLayout {
            title: title,
            data: &data,
            content: Website {
                content: Content {
                    data: data,
                    posts: posts,
                },
                data: data,
            },
        }
    }

    Content<'a>(
        data: &'a WebsiteDataDB,
        posts: &'a Vec<PostDB>,
    ) {
        @PostList {
            data: data,
            posts: posts,
        }
    }
}

