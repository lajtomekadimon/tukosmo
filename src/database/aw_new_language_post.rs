use postgres::{Client, NoTls, Error};

use crate::config::global::db_auth_string;


pub fn aw_new_language_post(
    lang_code: String,
    lang_ids: Vec<i64>,
    lang_names: Vec<String>,
) -> Result<i64, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT aw_new_language_post($1, $2, $3)",
        &[&lang_code, &lang_ids, &lang_names]
    )?;

    Ok(row.get(0))
}

