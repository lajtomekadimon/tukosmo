use postgres::{Client, NoTls, Error};
use serde_json::Value;

use crate::config::global::db_auth_string;


pub fn aj_query(
    json_input: Value
) -> Result<Value, Error> {
    let mut client = Client::connect(db_auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT aj_query($2::JSONB)",
        &[&json_input,]
    )?;

    Ok(row.get(0))
}

