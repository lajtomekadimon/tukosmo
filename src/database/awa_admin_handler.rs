use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::config::global::db_auth_string;
use crate::database::data::DataDB;


pub fn awa_admin_handler(
    session_id: Uuid,
    lang_code: String,
) -> Option<DataDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT awa_admin_handler($1, $2)",
            &[&session_id, &lang_code]
        ) {
            let data_struct: DataDB = row.get(0);
            Some(data_struct)
        } else {
            None
        }
    } else {
        None
    }
}
