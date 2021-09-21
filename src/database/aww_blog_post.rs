use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types::PostDB;


pub fn aww_blog_post(
    language_of_user: i64,
    permalink_value: String
) -> Option<PostDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT aww_blog_post($1, $2)",
            &[&language_of_user, &permalink_value]
        ) {
            row.get(0)
        } else {
            None
        }
    } else {
        None
    }
}
