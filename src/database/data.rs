
pub struct DataDB {
    pub user: UserDB,
}

pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub name: String,
}
