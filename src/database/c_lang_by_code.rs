use postgres::{Client, NoTls, Error};

use crate::config::global::db_auth_string;


pub fn c_lang_by_code(
    lang_code: String
) -> Result<bool, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT c_lang_by_code($1)",
        &[&lang_code,]
    )?;

    Ok(row.get(0))
}

