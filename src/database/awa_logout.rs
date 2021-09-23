use postgres::{Client, NoTls, Error};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub fn awa_logout(
    session_id: Uuid,
) -> Result<(), Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let _row = client.query_one(
        "SELECT awa_logout($1)",
        &[&session_id,]
    )?;

    Ok(())
}

