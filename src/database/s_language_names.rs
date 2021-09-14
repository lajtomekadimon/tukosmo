use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::NameDB;
use crate::database::rows;


pub fn s_language_names(
    language_of_user: i64,
    language_id: i64,
) -> Vec<NameDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {

        if let Ok(rows) = client.query(
            "SELECT * FROM s_language_names($1, $2)",
            &[&language_of_user, &language_id]
        ) {

            vec = rows::names::names(rows);

        }

        // TODO: Control the error!
    }

    vec
}

