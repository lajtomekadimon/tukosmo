use postgres_types::{ToSql, FromSql};


#[derive(Clone, Debug, ToSql, FromSql)]
pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub name: String,
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
pub struct PostDB {
    pub id: i64,
    pub trans_id: i64,
    pub lang: i64,
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
pub struct AdminDataDB {
    pub userd: UserDB,
    pub lang: LanguageDB,
    pub languages: Vec<LanguageDB>,
}

#[derive(Clone, Debug, ToSql, FromSql)]
pub struct WebsiteDataDB {
    pub userd: Option<UserDB>,
    pub lang: LanguageDB,
    pub languages: Vec<LanguageDB>,
}
