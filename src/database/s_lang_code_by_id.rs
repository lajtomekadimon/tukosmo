use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;


pub fn s_lang_code_by_id(
    lang_id: i64
) -> Option<String> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT s_language_code_by_id($1)",
            &[&lang_id,]
        ) {
            Some(row.get(0))
        } else {
            None
        }
    } else {
        None
    }
}

