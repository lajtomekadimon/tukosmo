use postgres::{Client, NoTls, Error, row};
use postgres_types::ToSql;

use crate::config::global::db_auth_string;


pub fn query_db<RequestType: ToSql + std::marker::Sync>(
    r: RequestType,
) -> Result<row::Row, Error> {

    match Client::connect(db_auth_string(), NoTls) {

        Ok(mut client) => client.query_one(
            "SELECT query_db($1)",
            &[&r,]
        ),

        Err(e) => Err(e),

    }

}

