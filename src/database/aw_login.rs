use postgres::{Client, NoTls, Error};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub fn aw_login(
    session_id: Uuid,
) -> Result<bool, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT aw_login($1)",
        &[&session_id,]
    )?;

    Ok(row.get(0))
}
