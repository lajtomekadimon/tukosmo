use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::PostDB;
use crate::database::rows;


pub fn awa_posts(
    language_of_user: i64
) -> Vec<PostDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {

        if let Ok(rows) = client.query(
            "SELECT * FROM awa_posts($1)",
            &[&language_of_user,]
        ) {

            vec = rows::posts::posts(rows);

        }

        // TODO: Control the error!
    }

    vec
}

