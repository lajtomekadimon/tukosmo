use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types::LanguageWithNamesDB;


pub fn awa_edit_language(
    language_id: i64,
    language_of_user: i64,
) -> Option<LanguageWithNamesDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT awa_edit_language($1, $2)",
            &[&language_id, &language_of_user]
        ) {
            row.get(0)
        } else {
            None
        }
    } else {
        None
    }
}
