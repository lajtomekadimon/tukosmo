use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::PostDB;
use crate::database::rows;


pub fn aww_blog_post(
    language_of_user: i64,
    permalink_value: String
) -> Option<PostDB> {
    let mut post: Option<PostDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {

        if let Ok(rows) = client.query(
            "SELECT * FROM aww_blog_post($1, $2)",
            &[&language_of_user, &permalink_value]
        ) {

            let vec = rows::posts::posts(rows);

            post = vec.first().cloned();

        }

        // TODO: Control the error!
    }

    post
}

