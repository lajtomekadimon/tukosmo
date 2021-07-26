use postgres::{Client, NoTls, Error};
use serde_json::Value;

use crate::database::auth_string::auth_string;


pub fn aj_query(
    json_input: Value
) -> Result<Value, Error> {
    let mut client = Client::connect(auth_string(), NoTls)?;

    let row = client.query_one(
        "SELECT aj_query($2::JSONB)",
        &[&json_input,]
    )?;

    Ok(row.get(0))
}

