use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types::PostDB;


pub fn awa_posts(
    language_of_user: i64,
    results_number: i64,
    page_number: i64,
) -> Vec<PostDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT * FROM awa_posts($1, $2, $3)",
            &[
                &language_of_user,
                &results_number,
                &page_number,
            ]
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
