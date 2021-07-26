use postgres::{Client, NoTls, Error};

use crate::database::auth_string::auth_string;


pub fn s_user_password_by_email(
    email_value: String
) -> Result<String, Error> {
    let mut client = Client::connect(auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT s_user_password_by_email($1)",
        &[&email_value,]
    )?;

    Ok(row.get(0))
}

