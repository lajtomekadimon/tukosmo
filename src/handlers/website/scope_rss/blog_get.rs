use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_identity::Identity;
use postgres_types::{ToSql, FromSql};
use rss;
use std::io;
use chrono::{DateTime, Utc};

use crate::config::global::Config;
use crate::files::static_files::{
    staticf_route,
    FAVICON_32X32,
};
use crate::handlers::{
    website::user_request::user_request,
    website::blog_get::rw_blog,
    website::scope_blog::post_get::rw_blog_post,
    files_get::r_file,
};
use crate::database::{
    types,
    query_db::{QueryFunction, query_db},
};
use crate::i18n::{
    t::t,
    error_website_route::error_website_route,
};


pub fn rw_rss_blog(
    lang_code: &str,
) -> String {
    "/{lang}/rss/blog".replace("{lang}", lang_code)
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgiRssBlog {
    pub req: types::WebsiteRequest,
    pub results: i64,
}

impl QueryFunction for WgiRssBlog {
    fn query(&self) -> &str {
        "SELECT ahw_g_rss_blog($1)"
    }
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WgoRssBlog {
    pub data: types::WebsiteDataDB,
    pub routes: Vec<types::RouteDB>,
    pub posts: Vec<types::PostDB>,
}


pub async fn blog_get(
    config: web::Data<Config>,
    codename: web::Data<String>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {

    let user_req = user_request(req, id);

    match query_db(
        &config,
        WgiRssBlog {
            req: user_req.clone(),
            results: 10,  // TODO: Add to Tukosmo.toml a custom number
        },
    ).await {

        Ok(row) => {

            let q: WgoRssBlog = row.get(0);
            let t = &t(&q.data.lang.code);

            let mut items: Vec<rss::Item> = Vec::new();

            for post in q.posts.iter() {
                let datetime = DateTime::parse_from_str(
                    &post.date,
                    "%Y-%m-%d %H:%M:%S%.f%#z"
                ).unwrap();
                let datetime_utc = datetime.with_timezone(&Utc);

                let mut dc_ext = rss::extension::dublincore::DublinCoreExtension::default();
                dc_ext.set_creators(vec![post.author_name.clone()]);
                dc_ext.set_languages(vec![post.lang.code.clone()]);

                let mut guid_value = rss::Guid::default();
                guid_value.set_permalink(true);
                guid_value.set_value("https://{domain}{route}"
                    .replace("{domain}", &config.server.domain)
                    .replace("{route}", &rw_blog_post(
                        &post.lang.code,
                        &post.permalink,
                    ))
                );

                let mut item = rss::Item::default();
                item.set_title(post.title.clone());
                item.set_link("https://{domain}{route}"
                    .replace("{domain}", &config.server.domain)
                    .replace("{route}", &rw_blog_post(
                        &post.lang.code,
                        &post.permalink,
                    ))
                );
                item.set_description(post.description.clone());
                //item.set_categories();  // TODO: tags
                if let Some(file_image) = &post.featured_image {
                    let mut enclosure = rss::Enclosure::default();
                    enclosure.set_url("https://{domain}{dir}"
                        .replace("{domain}", &config.server.domain)
                        .replace(
                            "{dir}",
                            &r_file(&file_image.name),
                        )
                    );
                    enclosure.set_length(file_image.bytes.to_string());
                    enclosure.set_mime_type("image/*");

                    item.set_enclosure(enclosure);
                }
                item.set_guid(guid_value);
                item.set_pub_date(datetime_utc.to_rfc2822());
                item.set_content(post.body.clone());
                item.set_dublin_core_ext(dc_ext);

                items.push(item);
            }

            // Create channel
            let channel = rss::ChannelBuilder::default()
                .title(
                    &format!(
                        "{a} - {b}",
                        a = q.data.website_title,
                        b = t.blog,
                    )
                )
                .link(
                    "https://{domain}{route}"
                        .replace("{domain}", &config.server.domain)
                        .replace("{route}", &rw_blog(&q.data.lang.code))
                )
                .description(q.data.website_subtitle)
                .language(q.data.lang.code.clone())
                .copyright(q.data.copyright_owner)
                //.ttl("10")  // number of minutes the channel can be cached
                .generator("Tukosmo".to_string())  // TODO: Add link?
                //.last_build_date()  // RFC822 timestamp
                .image(rss::Image {
                    url: staticf_route(FAVICON_32X32, &codename),
                    title: q.data.website_title,
                    description: None,
                    link: "https://{domain}{route}"
                        .replace("{domain}", &config.server.domain)
                        .replace("{route}", &rw_blog(&q.data.lang.code)),
                    width: Some("32".to_string()),
                    height: Some("32".to_string()),
                })
                .items(items)
                .build();

            // Write to the channel to a writer
            channel.write_to(io::sink()).unwrap();
            // Convert the channel to a string
            let rss_string = channel.to_string();

            HttpResponse::Ok()
                .content_type("application/rss+xml; charset=UTF-8")
                .body(rss_string)

        },

        Err(e) => error_website_route(&e, &user_req.lang_code),

    }

}

