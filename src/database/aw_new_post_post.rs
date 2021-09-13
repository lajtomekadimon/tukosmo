use postgres::{Client, NoTls, Error};

use crate::config::global::db_auth_string;


pub fn aw_new_post_post(
    post_id: i64,
    lang_code: String,
    title_value: String,
    description_value: String,
    body_value: String,
    permalink_value: String,
    author_id: i64,
    is_draft: bool,
) -> Result<i64, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT aw_new_post_post($1, $2, $3, $4, $5, $6, $7, $8)",
        &[
            &post_id,
            &lang_code,
            &title_value,
            &description_value,
            &body_value,
            &permalink_value,
            &author_id,
            &is_draft,
        ]
    )?;

    Ok(row.get(0))
}

