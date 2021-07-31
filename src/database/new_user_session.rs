use postgres::{Client, NoTls, Error};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub fn new_user_session(
    email_value: String,
    user_agent_value: String
) -> Result<Uuid, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT i_session(s_user_id_by_email($1), $2)",
        &[&email_value, &user_agent_value]
    )?;

    Ok(row.get(0))
}

