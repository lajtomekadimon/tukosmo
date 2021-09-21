use postgres::{Client, NoTls};
use uuid::Uuid;

use crate::config::global::db_auth_string;
use crate::database::types::WebsiteDataDB;


pub fn aww_website_handler(
    maybe_session_id: Option<Uuid>,
    lang_code: String,
) -> Option<WebsiteDataDB> {
    if let Ok(mut client) = Client::connect(db_auth_string(), NoTls) {
        match maybe_session_id {
            Some(session_id) => {
                if let Ok(row) = client.query_one(
                    "SELECT aww_website_handler($1, $2)",
                    &[&session_id, &lang_code]
                ) {
                    row.get(0)
                } else {
                    None
                }
            }

            None => {
                if let Ok(row) = client.query_one(
                    "SELECT aww_website_handler(NULL, $1)",
                    &[&lang_code,]
                ) {
                    row.get(0)
                } else {
                    None
                }
            }
        }
    } else {
        None
    }
}
