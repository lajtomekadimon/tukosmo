
pub struct DataDB {
    pub user: UserDB,
    pub languages: Vec<LanguageDB>,
}

pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub name: String,
}

pub struct LanguageDB {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub date: String,
    pub has_all_names: bool,
}
