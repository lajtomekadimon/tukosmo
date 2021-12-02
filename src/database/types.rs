use postgres_types::{ToSql, FromSql};
use uuid::Uuid;


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AdminRequest {
    pub session: Uuid,
    pub lang_code: String,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsiteRequest {
    pub session: Option<Uuid>,
    pub lang_code: String,
}

/*---*/

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub date: String,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct SessionDB {
    pub user_agent: String,
    pub date: String,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct FileDB {
    pub id: i64,
    pub name: String,
    pub ext: String,
    pub author: i64,
    pub author_name: String,
    pub date: String,
}
 
#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LanguageDB {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub original_name: String,
    pub date: String,
    pub has_all_names: bool,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct NameDB {
    pub name: String,
    pub lang: LanguageDB,
}
 
#[derive(Clone, Debug, ToSql, FromSql)]
pub struct LanguageWithNamesDB {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub original_name: String,
    pub date: String,
    pub has_all_names: bool,
    pub names: Vec<NameDB>,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct PostDB {
    pub id: i64,
    pub featured_image: Option<FileDB>,
    pub trans_id: i64,
    pub lang: LanguageDB,
    pub title: String,
    pub description: String,
    pub body: String,
    pub permalink: String,
    pub author: i64,
    pub author_name: String,
    pub translator: i64,
    pub translator_name: String,
    pub date: String,
    pub date_trans: String,
    pub draft: bool,
    pub deleted: bool,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct RouteDB {
    pub lang: LanguageDB,
    pub route: String,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct AdminDataDB {
    pub userd: UserDB,
    pub lang: LanguageDB,
    pub languages: Vec<LanguageDB>,
    pub website_title: String,
    pub website_subtitle: String,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsiteDataDB {
    pub userd: Option<UserDB>,
    pub lang: LanguageDB,
    pub languages: Vec<LanguageDB>,
    pub website_title: String,
    pub website_subtitle: String,
}
