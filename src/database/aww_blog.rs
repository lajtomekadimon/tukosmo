use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types::PostDB;


pub fn aww_blog(
    language_of_user: i64,
) -> Vec<PostDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT * FROM aww_blog($1)",
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
