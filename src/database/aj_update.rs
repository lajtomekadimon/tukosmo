use postgres::{Client, NoTls, Error};
use serde_json::Value;
use uuid::Uuid;

use crate::database::auth_string::auth_string;


pub fn aj_update(
    json_input: Value,
    session_id: Uuid
) -> Result<Value, Error> {
    let mut client = Client::connect(auth_string(), NoTls)?;

    let row = client.query_one(
        // TODO: Check if session has expired!!!
        "SELECT aj_update($1::JSONB, s_user_by_session($2))",
        &[&json_input, &session_id]
    )?;

    Ok(row.get(0))
}

