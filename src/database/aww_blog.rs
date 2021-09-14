use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::PostDB;
use crate::database::rows;


pub fn aww_blog(
    language_of_user: i64,
) -> Vec<PostDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {

        if let Ok(rows) = client.query(
            "SELECT * FROM aww_blog($1)",
            &[&language_of_user,]
        ) {

            vec = rows::posts::posts(rows);

        }

        // TODO: Control the error!
    }

    vec
}

