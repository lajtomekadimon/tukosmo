
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
