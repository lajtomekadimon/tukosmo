/*use postgres::{Client, NoTls};

use crate::config::global::db_auth_string;
use crate::database::types;




pub fn awa_edit_language(
    r: EditLanguageAR,
) -> Option<EditLanguageAH> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        if let Ok(row) = client.query_one(
            "SELECT awa_edit_language($1)",
            &[&r]
        ) {
            row.get(0)
        } else {
            None
        }
    } else {
        None
    }
}*/
