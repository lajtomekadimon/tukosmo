use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::data::LanguageDB;
use crate::database::rows;


pub fn s_current_language_by_code(
    lang_code: String
) -> Option<LanguageDB> {
    let mut lang: Option<LanguageDB> = None;

    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(rows) = client.query(
            "SELECT * FROM s_current_language_by_code($1)",
            &[&lang_code,]
        ) {

            let vec = rows::languages::languages(rows);

            lang = vec.first().cloned();

        }
        // TODO: Control the error!
    }

    lang
}

