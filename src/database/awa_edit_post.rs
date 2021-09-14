use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::PostDB;
use crate::database::rows;


pub fn awa_edit_post(
    post_id: i64,
    language_of_user: i64,
) -> Option<PostDB> {
    let mut post: Option<PostDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {

        if let Ok(rows) = client.query(
            "SELECT * FROM awa_edit_post($1, $2)",
            &[&post_id, &language_of_user]
        ) {

            let vec = rows::posts::posts(rows);

            post = vec.first().cloned();

        }

        // TODO: Control the error!
    }

    post
}

