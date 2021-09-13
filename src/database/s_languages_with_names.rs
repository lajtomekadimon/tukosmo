use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub struct LanguageDB {
    pub id: i64,
    pub name: String,
    pub trans_name: String,
}

pub fn s_languages_with_names(
    language_of_user: i64,
    language_id: i64,
) -> Vec<LanguageDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_languages_with_names($1, $2)",
            &[&language_of_user, &language_id]
        ) {
            for row in rows {
                let lang_id: i64 = row.get("id");
                let lang_name: String = row.get("name");
                let lang_trans_name: String = row.get("trans_name");

                vec.push(
                    LanguageDB {
                        id: lang_id,
                        name: lang_name,
                        trans_name: lang_trans_name,
                    }
                );                
            }
        }
    }

    vec
}

