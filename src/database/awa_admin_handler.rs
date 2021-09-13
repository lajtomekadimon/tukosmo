use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::config::global::db_auth_string;
use crate::database::data::UserDB;


pub fn awa_admin_handler(
    session_id: Uuid,
    language_of_user: i64,
) -> Option<UserDB> {
    let mut post_struct: Option<UserDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM awa_admin_handler($1, $2)",
            &[&session_id, &language_of_user]
        ) {
            for row in rows {
                let user_id: i64 = row.get("id");
                let user_email: String = row.get("email");
                let user_name: String = row.get("name");

                post_struct = Some(
                    UserDB {
                        id: user_id,
                        email: user_email,
                        name: user_name,
                    }
                );
            }
        }
        // TODO: Control the error!
    }

    post_struct
}

