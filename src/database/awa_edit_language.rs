use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::NameDB;


pub fn awa_edit_language(
    language_of_user: i64,
    language_id: i64,
) -> Vec<NameDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT awa_edit_language($1, $2)",
            &[&language_of_user, &language_id]
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
