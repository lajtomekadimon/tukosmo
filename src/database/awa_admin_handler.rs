use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::config::global::db_auth_string;
use crate::database::types::AdminDataDB;


pub fn awa_admin_handler(
    session_id: Uuid,
    lang_code: String,
) -> Option<AdminDataDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT awa_admin_handler($1, $2)",
            &[&session_id, &lang_code]
        ) {
            row.get(0)
        } else {
            None
        }
    } else {
        None
    }
}
