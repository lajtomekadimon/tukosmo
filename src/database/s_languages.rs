use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::LanguageDB;


pub fn s_languages(
    language_of_user: i64
) -> Vec<LanguageDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_languages($1)",
            &[&language_of_user,]
        ) {
            for row in rows {
                let lang_id: i64 = row.get("id");
                let lang_code: String = row.get("code");
                let lang_name: String = row.get("name");
                let date_value: String = row.get("date");
                let has_all_names: bool = row.get("has_all_names");

                vec.push(
                    LanguageDB {
                        id: lang_id,
                        code: lang_code,
                        name: lang_name,
                        date: date_value,
                        has_all_names: has_all_names,
                    }
                );                
            }
        }
        // TODO: Control the error!
    }

    vec
}

