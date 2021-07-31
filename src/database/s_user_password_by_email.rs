use postgres::{Client, NoTls, Error};

use crate::config::global::db_auth_string;


pub fn s_user_password_by_email(
    email_value: String
) -> Result<String, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT s_user_password_by_email($1)",
        &[&email_value,]
    )?;

    Ok(row.get(0))
}

