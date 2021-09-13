use postgres::{Client, NoTls, Error};
use uuid::Uuid;

use crate::config::global::db_auth_string;


pub fn d_session(
    session_id: Uuid,
) -> Result<(), Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let _row = client.query_one(
        "SELECT d_session($1)",
        &[&session_id,]
    )?;

    Ok(())
}

