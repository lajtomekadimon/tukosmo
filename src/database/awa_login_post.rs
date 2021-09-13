use postgres::{Client, NoTls, Error};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub fn awa_login_post(
    user_value: String,
    user_password: String,
    user_agent_value: String,
) -> Result<Uuid, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT awa_login_post($1, $2, $3)",
        &[&user_value, &user_password, &user_agent_value]
    )?;

    Ok(row.get(0))
}

