use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub name: String,
}

pub fn s_user_by_session_lang(
    session_id: Uuid,
    lang_code: String,
) -> Option<UserDB> {
    let mut post_struct: Option<UserDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_user_by_session_lang($1, s_language_id_by_code($2))",
            &[&session_id, &lang_code]
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

