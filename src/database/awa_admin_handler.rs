use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::config::global::db_auth_string;
use crate::database::data::UserDB;
use crate::database::rows;


pub fn awa_admin_handler(
    session_id: Uuid,
    language_of_user: i64,
) -> Option<UserDB> {
    let mut user: Option<UserDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM awa_admin_handler($1, $2)",
            &[&session_id, &language_of_user]
        ) {

            let vec = rows::users::users(rows);

            user = vec.first().cloned();

        }
        // TODO: Control the error!
    }

    user
}

