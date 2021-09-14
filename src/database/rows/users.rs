use postgres::row::Row;

use crate::database::data::UserDB;


pub fn users(
    rows: Vec<Row>
) -> Vec<UserDB> {
    let mut vec = Vec::new();

    for row in rows {
        let user_id: i64 = row.get("id");
        let user_email: String = row.get("email");
        let user_name: String = row.get("name");

        vec.push(
            UserDB {
                id: user_id,
                email: user_email,
                name: user_name,
            }
        );
    }

    vec
}

