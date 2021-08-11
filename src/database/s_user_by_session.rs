use postgres::{Client, NoTls, Error};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub fn s_user_by_session(
    session_id: Uuid
) -> Result<String, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT s_user_by_session($1)::TEXT",
        &[&session_id,]
    )?;

    Ok(row.get(0))
}

