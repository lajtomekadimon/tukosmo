use markup;

use crate::database::types::{WebsiteDataDB, RouteDB};


#[derive(Clone, Debug)]
pub struct ArticleOG {
    pub published_time: String,
    pub modified_time: String,
    /*
    pub section: String,
    pub tag: String,
    */
}


markup::define! {
    OpenGraphMeta<'a>(
        data: &'a WebsiteDataDB,
        routes: &'a Vec<RouteDB>,
        domain: &'a str,
        title: &'a str,
        description: &'a str,
        image: &'a str,
        article: &'a Option<ArticleOG>,
    ) {
        @for route in routes.iter() {
            @if &route.lang.code == &data.lang.code {
                meta[
                    property = "og:url",
                    content = "https://{domain}{route}"
                        .replace("{domain}", domain)
                        .replace("{route}", &route.route),
                ];
            }
        }

        meta[
            property = "og:site_name",
            content = &data.website_title,
        ];

        meta[
            property = "og:title",
            content = title,
        ];

        @if description != &"" {
            meta[
                property = "og:description",
                content = description,
            ];
        }

        @if image != &"" {
            meta[
                property = "og:image",
                content = image,
            ];
        }

        meta[
            property = "og:type",
            content = match article {
                Some(_) => "article",
                None => "website",
            },
        ];

        @if let Some(a) = article {
            meta[
                property = "article:published_time",
                content = &a.published_time,
            ];
            meta[
                property = "article:modified_time",
                content = &a.modified_time,
            ];
            /*
            meta[
                property = "article:section",
                content = "",
            ];
            meta[
                property = "article:tag",
                content = "",
            ];
            */
        }
    }
}

