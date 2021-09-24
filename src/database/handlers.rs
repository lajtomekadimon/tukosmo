use postgres_types::{ToSql, FromSql};
use uuid::Uuid;

use crate::database::types;


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AccountAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct DashboardAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguageAH {
    pub data: types::AdminDataDB,
    pub lang: types::LanguageDB,
    pub names: Vec<types::NameDB>,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditLanguagePostAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditPostAH {
    pub data: types::AdminDataDB,
    pub post: types::PostDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct EditPostPostAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct FilesAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LanguagesAH {
    pub data: types::AdminDataDB,
    pub languages: Vec<types::LanguageDB>,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LoginPostAH {
    pub data: types::AdminDataDB,
    pub session: Uuid,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LogoutAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewLanguageAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewLanguagePostAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewPostAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NewPostPostAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PagesAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PostsAH {
    pub data: types::AdminDataDB,
    pub posts: Vec<types::PostDB>,
    pub total_posts: i64,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct ServerAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SessionsAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct StatisticsAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct TukosmoAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct UsersAH {
    pub data: types::AdminDataDB,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsiteAH {
    pub data: types::AdminDataDB,
}


/*----------------------*/


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct BlogWH {
    pub data: types::WebsiteDataDB,
    pub posts: Vec<types::PostDB>,
    pub posts_in_total: i64,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct BlogPostWH {
    pub data: types::WebsiteDataDB,
    pub post: types::PostDB,
    //pub post_translations: Vec<types::PostDB>,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PageWH {
    pub data: types::WebsiteDataDB,
    //pub page: types::PageDB,
    //pub page_translations: Vec<types::PageDB>,
}
