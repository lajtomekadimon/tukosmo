
#[derive(Clone)]
pub struct DataDB {
    pub user: UserDB,
    pub lang: CurrentLanguageDB,
    pub languages: Vec<LanguageDB>,
}

#[derive(Clone)]
pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub name: String,
}
 
#[derive(Clone)]
pub struct LanguageDB {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub date: String,
    pub has_all_names: bool,
}

#[derive(Clone)]
pub struct CurrentLanguageDB {
    pub id: i64,
    pub code: String,
    pub name: String,
}

#[derive(Clone)]
pub struct PostDB {
    pub id: i64,
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
