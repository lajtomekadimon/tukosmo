use postgres::{Client, NoTls, Error};

use crate::config::global::db_auth_string;


pub fn as_clean_sessions() -> Result<(), Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let _row = client.simple_query("SELECT as_clean_sessions()")?;

    Ok(())
}

