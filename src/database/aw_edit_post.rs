use postgres::{Client, NoTls, Error};

use crate::config::global::db_auth_string;


pub fn aw_edit_post(
    post_id: i64,
    lang_code: String,
    title_value: String,
    description_value: String,
    body_value: String,
    permalink_value: String,
    is_draft: bool,
    is_deleted: bool,
    translator_id: i64,
) -> Result<i64, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT aw_edit_post($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        &[
            &post_id,
            &lang_code,
            &title_value,
            &description_value,
            &body_value,
            &permalink_value,
            &is_draft,
            &is_deleted,
            &translator_id,
        ]
    )?;

    Ok(row.get(0))
}

