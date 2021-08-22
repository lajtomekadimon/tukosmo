use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub struct LanguageDB {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub date: String,
    pub has_all_names: bool,
}

pub fn s_languages(
    lang_code: String
) -> Vec<LanguageDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_languages(s_lang_id_by_code($1))",
            &[&lang_code,]
        ) {
            for row in rows {
                let lang_id: i64 = row.get("tlc_id");
                let lang_code: String = row.get("tlc_code");
                let lang_name: String = row.get("tl_name");
                let date_value: String = row.get("tl_date");
                let has_all_names: bool = row.get("tl_has_all_names");

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
    }

    vec
}

