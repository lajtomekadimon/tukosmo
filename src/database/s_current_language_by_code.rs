use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::CurrentLanguageDB;


pub fn s_current_language_by_code(
    lang_code: String
) -> Option<CurrentLanguageDB> {
    let mut lang_struct: Option<CurrentLanguageDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_current_language_by_code($1)",
            &[&lang_code,]
        ) {
            for row in rows {
                let lang_id: i64 = row.get("id");
                let lang_code: String = row.get("code");
                let lang_name: String = row.get("name");

                lang_struct = Some(
                    CurrentLanguageDB {
                        id: lang_id,
                        code: lang_code,
                        name: lang_name,
                    }
                );
            }
        }
        // TODO: Control the error!
    }

    lang_struct
}

