use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types::LanguageDB;
use crate::database::rows;


pub fn s_languages(
    language_of_user: i64
) -> Vec<LanguageDB> {
    let mut vec = Vec::new();

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {

        if let Ok(rows) = client.query(
            "SELECT * FROM s_languages($1)",
            &[&language_of_user,]
        ) {

            vec = rows::languages::languages(rows);

        }

        // TODO: Control the error!
    }

    vec
}

