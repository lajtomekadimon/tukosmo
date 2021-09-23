use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types::LanguageDB;


pub fn s_languages(
    language_of_user: i64
) -> Vec<LanguageDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT * FROM s_languages($1)",
            &[&language_of_user,]
        ) {
            if let Some(vec) = row.get(0) {
                vec
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    }
}
